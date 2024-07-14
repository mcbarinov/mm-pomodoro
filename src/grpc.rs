use std::process;
use std::sync::Arc;
use std::time::Duration;

use hyper_util::rt::TokioIo;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{Mutex, Notify};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tonic::{Request, Response, Status};
use tower::service_fn;

use crate::timer_grpc::timer_service_client::TimerServiceClient;
use crate::timer_grpc::timer_service_server::{TimerService, TimerServiceServer};
use crate::timer_grpc::{Empty, State};

pub async fn connect_client_or_exit() -> TimerServiceClient<Channel> {
    match Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(|_: Uri| async {
            let path = "/tmp/ptimer.sock";
            Ok::<_, std::io::Error>(TokioIo::new(UnixStream::connect(path).await?))
        }))
        .await
    {
        Ok(channel) => TimerServiceClient::new(channel),
        Err(_) => {
            println!("is ptimer running?");
            process::exit(0)
        }
    }
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

pub async fn start_grpc_server(interval: u64) -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/ptimer.sock";
    let _ = tokio::fs::remove_file(path).await; // TODO: ?
    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    let notify = Arc::new(Notify::new());
    let state = Arc::new(Mutex::new(State::new(interval)));

    tokio::spawn({
        let notify_clone = Arc::clone(&notify);
        let state_clone = Arc::clone(&state);
        async move {
            loop {
                // Check if we need to stop the timer
                if state_clone.lock().await.need_to_stop() {
                    notify_clone.notify_one();
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });

    println!("Listening on: {}", path);
    Server::builder()
        .add_service(TimerServiceServer::new(RpcService::new(state)))
        .serve_with_incoming_shutdown(uds_stream, async {
            notify.notified().await;
            println!("Shutting down...");
        })
        .await?;

    Ok(())
}
