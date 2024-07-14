use clap::{Parser, Subcommand};

use crate::command;
use crate::config::Config;

/// Pomodoro timer
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    fn command(&self) -> Commands {
        self.command.clone().unwrap_or(Commands::Status)
    }
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Start a new pomodoro timer
    Start {
        #[arg(help = "Interval in seconds")]
        interval: u64,
    },

    /// Show the current status of the timer
    Status,

    /// Pause the current timer
    Pause,

    /// Resume the current timer
    Resume,

    /// Stop the current timer
    Stop,
}

pub fn run(config: &Config) {
    let cli = Cli::parse();
    match cli.command() {
        Commands::Status => command::status_run(config),
        Commands::Pause => command::pause_run(config),
        Commands::Resume => command::resume_run(config),
        Commands::Stop => command::stop_run(config),
        Commands::Start { interval } => command::start_run(interval, config),
    }
}
