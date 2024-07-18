use crate::config::Config;
use crate::db::delete_history;

pub fn run(config: &Config, id: u32) {
    delete_history(config, id).unwrap();
}
