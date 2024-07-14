use std::error;

use crate::grpc::connect_client_or_exit;

pub fn run() {
    tokio::runtime::Runtime::new().unwrap().block_on(run_()).unwrap();
}

async fn run_() -> Result<(), Box<dyn error::Error>> {
    let mut client = connect_client_or_exit().await;

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
