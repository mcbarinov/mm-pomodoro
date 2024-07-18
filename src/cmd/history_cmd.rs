use crate::config::Config;
use crate::history::print_history;

pub fn run(config: &Config, show_all: bool, print_id: bool) {
    print_history(config, show_all, print_id);
}
