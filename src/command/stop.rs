use crate::config::Config;
use crate::grpc::connect_client_or_exit;

pub fn run(config: &Config) {
    tokio::runtime::Runtime::new().unwrap().block_on(run_(config));
}

async fn run_(config: &Config) {
    let mut client = connect_client_or_exit(config).await;
    match client.stop(tonic::Request::new(crate::timer_grpc::Empty {})).await {
        Ok(state) => {
            state.into_inner().pretty_print();
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
