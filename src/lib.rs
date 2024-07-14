pub use cli::run;

mod cli;
mod command;
mod grpc;
mod notification;
mod state;

pub mod timer_grpc {
    tonic::include_proto!("timer");
}
