use std::fs::File;

use daemonize::Daemonize;

use crate::config::Config;
use crate::grpc::start_grpc_server;
use crate::notification::send_notification;

pub fn run(interval: u64, config: &Config) {
    let stdout = File::create(&config.daemon_stdout).unwrap();
    let stderr = File::create(&config.daemon_stderr).unwrap();
    let daemon = Daemonize::new().stdout(stdout).stderr(stderr).pid_file(&config.daemon_pidfile);

    println!("starting a new timer with interval: {}", interval);
    match daemon.start() {
        Ok(_) => {
            tokio::runtime::Runtime::new().unwrap().block_on(start_grpc_server(interval, config)).unwrap();
            send_notification();
        }
        Err(e) => {
            eprintln!("Error, {}", e)
        }
    }
}
