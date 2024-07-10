use clap::{Parser, Subcommand};

use crate::command;

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

pub fn run() {
    let cli = Cli::parse();
    match cli.command() {
        Commands::Status => command::status_run(),
        Commands::Pause => command::pause_run(),
        Commands::Resume => command::resume_run(),
        Commands::Stop => command::stop_run(),
        Commands::Start { interval } => command::start_run(interval),
    }
}
