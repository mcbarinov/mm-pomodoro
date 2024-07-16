use crate::config::Config;
use crate::db::query_history;

pub fn run(config: &Config) {
    let history = query_history(config).unwrap();
    for h in history {
        h.pretty_print();
    }
}
