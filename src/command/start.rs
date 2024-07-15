use std::fs::File;
use std::time::Duration;

use daemonize::Daemonize;
use humantime::format_duration;

use crate::config::Config;
use crate::grpc::start_grpc_server;
use crate::notification::send_notification;

pub fn run(duration: Duration, config: &Config) {
    let stdout = File::create(&config.daemon_stdout).unwrap();
    let stderr = File::create(&config.daemon_stderr).unwrap();
    let daemon = Daemonize::new().stdout(stdout).stderr(stderr).pid_file(&config.daemon_pidfile);

    println!("starting a new timer with duration: {}", format_duration(duration));
    match daemon.start() {
        Ok(_) => {
            tokio::runtime::Runtime::new().unwrap().block_on(start_grpc_server(duration.as_secs(), config)).unwrap();
            send_notification();
        }
        Err(e) => {
            eprintln!("Error, {}", e)
        }
    }
}
