use std::error;

use chrono::{TimeZone, Utc};

use crate::grpc::connect_client_or_exit;

pub fn run() {
    tokio::runtime::Runtime::new().unwrap().block_on(run_()).unwrap();
}

async fn run_() -> Result<(), Box<dyn error::Error>> {
    let mut client = connect_client_or_exit().await;

    match client.stop(tonic::Request::new(crate::timer_grpc::Empty {})).await {
        Ok(status) => {
            let status = status.into_inner();
            let r = Utc.timestamp_opt(status.started_at, 0).unwrap();
            let message = format!("started at: {}\nseconds remaining: {}", r, status.seconds_remaining);
            println!("{}", message);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    Ok(())
}
