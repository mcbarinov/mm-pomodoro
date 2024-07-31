use crate::config::Config;
use crate::grpc::connect_client;

pub async fn run(config: &Config) {
    let mut client = connect_client(config).await.unwrap_or_else(|| {
        println!("mm-pomodoro is not running");
        std::process::exit(1);
    });
    match client.pause(tonic::Request::new(crate::timer_grpc::Empty {})).await {
        Ok(state) => {
            state.into_inner().pretty_print();
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
