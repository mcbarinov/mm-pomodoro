use std::future::Future;

use clap::{Parser, Subcommand};
use tokio::runtime::Runtime;

use crate::cmd;
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
    #[command(name = "new", visible_alias = "n")]
    New {
        #[arg(
            help = "Durations. For example, 1h20m30s. The default unit is minutes, so 30 means 30 minutes.",
            env = "PTIMER_NEW_DURATION"
        )]
        duration: String,
    },

    /// Show the current status of the timer
    Status,

    /// Pause the current timer
    #[command(name = "pause", visible_alias = "p")]
    Pause,

    /// Resume the current timer
    #[command(name = "resume", visible_alias = "r")]
    Resume,

    /// Stop the current timer
    #[command(name = "stop")]
    Stop,

    /// Show the history of finished timers
    #[command(name = "history", visible_alias = "h")]
    History {
        #[arg(
            short,
            long,
            help = "Show all recent history. Buy default, only today's history is shown.",
            default_value_t = false
        )]
        all: bool,

        #[arg(long, help = "Print the ID of the history")]
        print_id: bool,
    },

    /// Delete a history by ID
    DeleteHistory { id: u32 },
}

pub fn run(config: &Config) {
    let cli = Cli::parse();
    match cli.command() {
        Commands::Status => run_async(cmd::status_cmd::run(config)),
        Commands::Pause => run_async(cmd::pause_cmd::run(config)),
        Commands::Resume => run_async(cmd::resume_cmd::run(config)),
        Commands::Stop => run_async(cmd::stop_cmd::run(config)),
        Commands::History { all, print_id } => cmd::history_cmd::run(config, all, print_id),
        Commands::New { mut duration } => {
            // If the duration is a number, it's in minutes
            if duration.parse::<u64>().is_ok() {
                duration = format!("{duration}m");
            }
            let duration = humantime::parse_duration(&duration).expect("Invalid duration, use 1h20m30s format");
            cmd::new_cmd::run(duration, config);
        }
        Commands::DeleteHistory { id } => cmd::delete_history_cmd::run(config, id),
    }
}

fn run_async<F, T>(f: F) -> T
where
    F: Future<Output = T>,
{
    let rt = Runtime::new().expect("Failed to create runtime");
    rt.block_on(f)
}
