use crate::config::Config;
use crate::grpc::connect_client_or_exit;

pub async fn run(config: &Config) {
    let mut client = connect_client_or_exit(config).await;
    match client.pause(tonic::Request::new(crate::timer_grpc::Empty {})).await {
        Ok(state) => {
            state.into_inner().pretty_print();
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
