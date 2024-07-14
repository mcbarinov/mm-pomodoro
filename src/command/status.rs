use std::error;

use crate::config::Config;
use crate::grpc::connect_client_or_exit;

pub fn run(config: &Config) {
    tokio::runtime::Runtime::new().unwrap().block_on(run_(config)).unwrap();
}

async fn run_(config: &Config) -> Result<(), Box<dyn error::Error>> {
    let mut client = connect_client_or_exit(config).await;

    match client.status(tonic::Request::new(crate::timer_grpc::Empty {})).await {
        Ok(state) => {
            state.into_inner().pretty_print();
        }
        Err(err) => {
            dbg!(err);
        }
    }

    Ok(())
}
