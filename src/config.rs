#[derive(Debug, Clone)]
pub struct Config {
    pub daemon_pidfile: String,
    pub daemon_stdout: String,
    pub daemon_stderr: String,
    pub grpc_uds_path: String,
}

impl Config {
    pub fn new() -> Self {
        if cfg!(debug_assertions) {
            Self {
                daemon_pidfile: "/tmp/ptimer_dev.pid".to_string(),
                daemon_stdout: "/tmp/ptimer_dev.out".to_string(),
                daemon_stderr: "/tmp/ptimer_dev.err".to_string(),
                grpc_uds_path: "/tmp/ptimer_dev.sock".to_string(),
            }
        } else {
            Self {
                daemon_pidfile: "/tmp/ptimer.pid".to_string(),
                daemon_stdout: "/tmp/ptimer.out".to_string(),
                daemon_stderr: "/tmp/ptimer.err".to_string(),
                grpc_uds_path: "/tmp/ptimer.sock".to_string(),
            }
        }
    }
}
