use std::sync::Arc;
use std::time::Duration;

use hyper_util::rt::TokioIo;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{Mutex, Notify};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tonic::{Request, Response, Status};
use tower::service_fn;

use crate::config::Config;
use crate::db::insert_history;
use crate::timer_grpc::timer_service_client::TimerServiceClient;
use crate::timer_grpc::timer_service_server::{TimerService, TimerServiceServer};
use crate::timer_grpc::{Empty, State};

pub async fn connect_client(config: &Config) -> Option<TimerServiceClient<Channel>> {
    match connect_channel(&config.grpc_uds_path).await {
        Ok(channel) => Some(TimerServiceClient::new(channel)),
        Err(_) => None,
    }
}

async fn connect_channel(uds_path: &str) -> Result<Channel, tonic::transport::Error> {
    let channel = Endpoint::try_from(format!("http://[::]:50051/{}", uds_path))?
        .connect_with_connector(service_fn(move |u: Uri| async move {
            Ok::<_, std::io::Error>(TokioIo::new(UnixStream::connect(u.path()).await?))
        }))
        .await?;
    Ok(channel)
}

#[derive(Debug, Clone)]
pub struct RpcService {
    state: Arc<Mutex<State>>,
}

impl RpcService {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TimerService for RpcService {
    async fn status(&self, _request: Request<Empty>) -> Result<Response<State>, Status> {
        Ok(Response::new(*self.state.lock().await))
    }

    async fn pause(&self, _request: Request<Empty>) -> Result<Response<State>, Status> {
        self.state.lock().await.pause();
        Ok(Response::new(*self.state.lock().await))
    }

    async fn resume(&self, _request: Request<Empty>) -> Result<Response<State>, Status> {
        self.state.lock().await.resume();
        Ok(Response::new(*self.state.lock().await))
    }

    async fn stop(&self, _request: Request<Empty>) -> Result<Response<State>, Status> {
        self.state.lock().await.stop();
        Ok(Response::new(*self.state.lock().await))
    }
}

pub async fn start_grpc_server(interval: u64, config: &Config) -> Result<(), anyhow::Error> {
    let _ = tokio::fs::remove_file(&config.grpc_uds_path).await; // TODO: ?
    let uds = UnixListener::bind(&config.grpc_uds_path)?;
    let uds_stream = UnixListenerStream::new(uds);

    let notify = Arc::new(Notify::new());
    let state = Arc::new(Mutex::new(State::new(interval)));

    tokio::spawn({
        let notify_clone = Arc::clone(&notify);
        let state_clone = Arc::clone(&state);
        let config_clone = config.clone();
        async move {
            loop {
                // Check if we need to stop the timer
                let state = state_clone.lock().await;

                if state.is_stopped() {
                    notify_clone.notify_one();
                    break;
                } else if state.is_truely_finished() {
                    notify_clone.notify_one();
                    // Create history only if the timer is finished, not stopped
                    insert_history(&config_clone, state.started_at, state.duration).unwrap();
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    println!("Listening on: {}", &config.grpc_uds_path);
    Server::builder()
        .add_service(TimerServiceServer::new(RpcService::new(state)))
        .serve_with_incoming_shutdown(uds_stream, async {
            notify.notified().await;
            println!("Shutting down...");
        })
        .await?;

    Ok(())
}
