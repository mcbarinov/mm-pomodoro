pub use cli::run;

mod cli;
mod command;

pub mod timer_grpc {
    tonic::include_proto!("timer");
}
