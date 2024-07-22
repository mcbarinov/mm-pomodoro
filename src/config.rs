use std::{env, fs};

#[derive(Debug, Clone)]
pub struct Config {
    pub debug_mode: bool,
    pub daemon_pidfile: String,
    pub daemon_stdout: String,
    pub daemon_stderr: String,
    pub grpc_uds_path: String,
    pub db_path: String,
}

impl Config {
    pub fn new() -> Self {
        let home_dir = env::var("HOME").expect("Can't get HOME env");
        let mut app_dir = format!("{home_dir}/.local/ptimer");
        let debug_mode = cfg!(debug_assertions);
        if debug_mode {
            app_dir = format!("{app_dir}-dev");
        }
        fs::create_dir_all(&app_dir).unwrap_or_else(|_| panic!("Can't create app dir: {app_dir}"));
        Self {
            daemon_pidfile: format!("{app_dir}/ptimer.pid"),
            daemon_stdout: format!("{app_dir}/stdout.log"),
            daemon_stderr: format!("{app_dir}/stderr.log"),
            grpc_uds_path: format!("{app_dir}/grpc.sock"),
            db_path: format!("{app_dir}/ptimer.db"),
            debug_mode,
        }
    }
}
