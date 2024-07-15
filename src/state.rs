use std::time::Duration;

use chrono::{Local, TimeZone};

use crate::timer_grpc::State;

impl State {
    pub fn new(interval: u64) -> Self {
        Self {
            started_at: Local::now().timestamp(),
            finish_at: (Local::now() + Duration::from_secs(interval)).timestamp(),
            stopped: false,
            paused: false,
            paused_seconds: 0,
        }
    }

    pub fn need_to_stop(&self) -> bool {
        self.stopped || (Local::now().timestamp() > self.finish_at && !self.paused)
    }

    pub fn pause(&mut self) {
        if self.paused {
            return;
        }
        self.paused = true;
        self.paused_seconds = self.finish_at - Local::now().timestamp();
    }

    pub fn resume(&mut self) {
        if !self.paused {
            return;
        }
        self.paused = false;
        self.finish_at = (Local::now() + Duration::from_secs(self.paused_seconds as u64)).timestamp();
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn pretty_print(&self) {
        let started_at = Local.timestamp_opt(self.started_at, 0).unwrap();
        let finish_at = Local.timestamp_opt(self.finish_at, 0).unwrap();

        if self.stopped {
            println!("stopped! started at: {}", started_at);
        } else if self.paused {
            let left = humantime::format_duration(Duration::from_secs(self.paused_seconds as u64));
            println!("paused! {} left, started at: {}, finish at: {}", left, started_at, finish_at);
        } else {
            let remaining_seconds = self.finish_at - Local::now().timestamp();
            let humanized_duration = humantime::format_duration(Duration::from_secs(remaining_seconds as u64));
            println!("{} left, started at: {}, finish at: {}", humanized_duration, started_at, finish_at);
        }
    }
}
