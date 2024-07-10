use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

use daemonize::Daemonize;
use tokio::net::UnixListener;
use tokio::sync::Notify;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::timer_grpc::timer_service_server::{TimerService, TimerServiceServer};
use crate::timer_grpc::{Empty, TimerStatus};

#[derive(Debug, Default)]
pub struct RpcService {}

#[tonic::async_trait]
impl TimerService for RpcService {
    async fn status(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(TimerStatus { status: "status".to_string() }))
    }

    async fn pause(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(TimerStatus { status: "pause".to_string() }))
    }

    async fn resume(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(TimerStatus { status: "resume".to_string() }))
    }

    async fn stop(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(TimerStatus { status: "stop".to_string() }))
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

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(interval)).await;
        notify_clone.notify_one();
        println!("timer!");
    });

    println!("Listening on: {}", path);
    Server::builder()
        .add_service(TimerServiceServer::new(RpcService {}))
        .serve_with_incoming_shutdown(uds_stream, async {
            notify.notified().await;
            println!("Shutting down...");
        })
        .await?;

    Ok(())
}
