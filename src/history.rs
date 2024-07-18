use chrono::Local;

use crate::config::Config;
use crate::db::query_history;

pub fn print_history(config: &Config, show_all: bool) {
    let history = query_history(config).unwrap();
    let now = Local::now().date_naive();
    let today_history: Vec<_> = history.iter().filter(|h| h.started_at.date_naive() == now).collect();

    if !show_all && !today_history.is_empty() {
        println!("Today's history:");
        for h in today_history {
            h.pretty_print();
        }
    } else {
        for h in history {
            h.pretty_print();
        }
    }
}
