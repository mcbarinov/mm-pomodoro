use std::{env, fs};

#[derive(Debug, Clone)]
pub struct Config {
    pub daemon_pidfile: String,
    pub daemon_stdout: String,
    pub daemon_stderr: String,
    pub grpc_uds_path: String,
    pub db_path: String,
}

impl Config {
    pub fn new() -> Self {
        let home_dir = env::var("HOME").expect("Can't get HOME env");
        let mut app_dir = format!("{}/.local/ptimer", home_dir);
        if cfg!(debug_assertions) {
            app_dir = format!("{app_dir}-dev");
        }
        fs::create_dir_all(&app_dir).unwrap_or_else(|_| panic!("Can't create app dir: {}", &app_dir));
        Self {
            daemon_pidfile: format!("{}/ptimer.pid", app_dir),
            daemon_stdout: format!("{}/stdout.log", app_dir),
            daemon_stderr: format!("{}/stderr.log", app_dir),
            grpc_uds_path: format!("{}/grpc.sock", app_dir),
            db_path: format!("{}/ptimer.db", app_dir),
        }
    }
}
