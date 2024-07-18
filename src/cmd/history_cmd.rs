use crate::config::Config;
use crate::history::print_history;

pub fn run(config: &Config, show_all: bool) {
    print_history(config, show_all);
}
