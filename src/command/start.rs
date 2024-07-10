use std::fs::File;

use daemonize::Daemonize;

pub fn run(interval: u64) {
    println!("start a new timer with interval: {}", interval);
    let stdout = File::create("/tmp/ptimer.out").unwrap();
    let stderr = File::create("/tmp/ptimer.err").unwrap();
    let daemon = Daemonize::new().stdout(stdout).stderr(stderr).pid_file("/tmp/ptimer.pid");
    match daemon.start() {
        Ok(_) => {
            println!("Daemon started");
            std::thread::sleep(std::time::Duration::from_secs(interval));
        }
        Err(e) => {
            eprintln!("Error, {}", e)
        }
    }
}
