use crate::config::Config;

mod cli;
mod cmd;
mod config;
mod db;
mod grpc;
mod history;
mod notification;
mod state;

pub mod timer_grpc {
    tonic::include_proto!("timer");
}

pub fn run() {
    let config = Config::new();
    cli::run(&config);
}
