use std::fs::File;

use daemonize::Daemonize;

use crate::grpc::start_grpc_server;
use crate::notification::send_notification;

pub fn run(interval: u64) {
    let stdout = File::create("/tmp/ptimer.out").unwrap();
    let stderr = File::create("/tmp/ptimer.err").unwrap();
    let daemon = Daemonize::new().stdout(stdout).stderr(stderr).pid_file("/tmp/ptimer.pid");

    println!("starting a new timer with interval: {}", interval);
    match daemon.start() {
        Ok(_) => {
            tokio::runtime::Runtime::new().unwrap().block_on(start_grpc_server(interval)).unwrap();
            send_notification();
        }
        Err(e) => {
            eprintln!("Error, {}", e)
        }
    }
}
