use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Local};
use daemonize::Daemonize;
use tokio::net::UnixListener;
use tokio::sync::{Mutex, Notify};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::notification::send_notification;
use crate::timer_grpc::timer_service_server::{TimerService, TimerServiceServer};
use crate::timer_grpc::{Empty, TimerStatus};

#[derive(Debug, Clone)]
pub struct RpcService {
    state: Arc<Mutex<State>>,
}

#[derive(Debug)]
pub struct State {
    started_at: DateTime<Local>,
    finished_at: DateTime<Local>,
    stopped: bool,
    paused: bool,
    paused_seconds: i64, // how many seconds remaining when paused
}

impl State {
    pub fn new(interval: u64) -> Self {
        Self {
            started_at: Local::now(),
            finished_at: Local::now() + Duration::from_secs(interval),
            stopped: false,
            paused: false,
            paused_seconds: 0,
        }
    }

    pub fn need_to_stop(&self) -> bool {
        self.stopped || (Local::now() > self.finished_at && !self.paused)
    }

    pub fn pause(&mut self) {
        if self.paused {
            return;
        }
        self.paused = true;
        self.paused_seconds = (self.finished_at - Local::now()).num_seconds();
    }

    pub fn resume(&mut self) {
        if !self.paused {
            return;
        }
        self.paused = false;
        self.finished_at = Local::now() + Duration::from_secs(self.paused_seconds as u64);
    }

    pub fn timer_status(&self) -> TimerStatus {
        let seconds_remaining = if self.paused { self.paused_seconds } else { (self.finished_at - Local::now()).num_seconds() };
        TimerStatus { started_at: self.started_at.timestamp(), seconds_remaining, paused: self.paused }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

impl RpcService {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TimerService for RpcService {
    async fn status(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        Ok(Response::new(self.state.lock().await.timer_status()))
    }

    async fn pause(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        self.state.lock().await.pause();
        Ok(Response::new(self.state.lock().await.timer_status()))
    }

    async fn resume(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        self.state.lock().await.resume();
        Ok(Response::new(self.state.lock().await.timer_status()))
    }

    async fn stop(&self, _request: Request<Empty>) -> Result<Response<TimerStatus>, Status> {
        println!("stop");
        self.state.lock().await.stop();
        Ok(Response::new(self.state.lock().await.timer_status()))
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

    let state = Arc::new(Mutex::new(State::new(interval)));
    let state_clone = Arc::clone(&state);

    tokio::spawn(async move {
        loop {
            // Check if we need to stop the timer
            if state_clone.lock().await.need_to_stop() {
                notify_clone.notify_one();
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
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
