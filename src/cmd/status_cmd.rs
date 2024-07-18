use crate::config::Config;
use crate::grpc::connect_client;
use crate::history::print_history;

pub async fn run(config: &Config) {
    let mut client = connect_client(config).await.unwrap_or_else(|| {
        println!("ptimer is not running");
        print_history(config, false, false);
        std::process::exit(0);
    });
    match client.status(tonic::Request::new(crate::timer_grpc::Empty {})).await {
        Ok(state) => {
            state.into_inner().pretty_print();
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
