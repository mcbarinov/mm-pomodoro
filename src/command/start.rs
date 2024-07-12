use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Local};
use daemonize::Daemonize;
use tokio::net::UnixListener;
use tokio::sync::Notify;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::notification::send_notification;
use crate::timer_grpc::timer_service_server::{TimerService, TimerServiceServer};
use crate::timer_grpc::{Empty, TimerStatus};

#[derive(Debug, Clone)]
pub struct RpcService {
    state: Arc<State>,
}

#[derive(Debug)]
pub struct State {
    pub started_at: DateTime<Local>,
    pub finished_at: DateTime<Local>,
}

impl State {
    pub fn new(interval: u64) -> Self {
        Self { started_at: Local::now(), finished_at: Local::now() + Duration::from_secs(interval) }
    }

    pub fn timer_status(&self) -> TimerStatus {
        let diff = self.finished_at - Local::now();
        let diff_in_seconds = diff.num_seconds();
        TimerStatus { started_at: self.started_at.timestamp(), seconds_remaining: diff_in_seconds }
    }
}

impl RpcService {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TimerService for RpcService {
    async fn status(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(self.state.timer_status()))
    }

    async fn pause(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(self.state.timer_status()))
    }

    async fn resume(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(self.state.timer_status()))
    }

    async fn stop(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(self.state.timer_status()))
    }
}

pub fn run(interval: u64) {
    println!("start a new timer with interval: {}", interval);
    let stdout = File::create("/tmp/ptimer.out").unwrap();
    let stderr = File::create("/tmp/ptimer.err").unwrap();
    let daemon = Daemonize::new().stdout(stdout).stderr(stderr).pid_file("/tmp/ptimer.pid");

    match daemon.start() {
        Ok(_) => {
            println!("Daemon started");
            tokio::runtime::Runtime::new().unwrap().block_on(start_grpc_server(interval)).unwrap();
            send_notification();
            println!("done!");
        }
        Err(e) => {
            eprintln!("Error, {}", e)
        }
    }
}

async fn start_grpc_server(interval: u64) -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/ptimer.sock";
    let _ = tokio::fs::remove_file(path).await; // TODO: ?
    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    let notify = Arc::new(Notify::new());
    let notify_clone = Arc::clone(&notify);

    let state = Arc::new(State::new(interval));
    let state_clone = Arc::clone(&state);

    let rpc_service = RpcService::new(Arc::clone(&state));

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("tick");
            if Local::now() > state_clone.finished_at {
                notify_clone.notify_one();
                break;
            }
        }
    });

    println!("Listening on: {}", path);
    Server::builder()
        .add_service(TimerServiceServer::new(rpc_service))
        .serve_with_incoming_shutdown(uds_stream, async {
            notify.notified().await;
            println!("Shutting down...");
        })
        .await?;

    Ok(())
}
