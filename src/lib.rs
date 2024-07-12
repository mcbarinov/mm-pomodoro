pub use cli::run;

mod cli;
mod command;
mod notification;

pub mod timer_grpc {
    tonic::include_proto!("timer");
}
