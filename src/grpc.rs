use std::process;

use hyper_util::rt::TokioIo;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

use crate::timer_grpc::timer_service_client::TimerServiceClient;

pub async fn connect_client_or_exit() -> TimerServiceClient<Channel> {
    match Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(|_: Uri| async {
            let path = "/tmp/ptimer.sock";
            Ok::<_, std::io::Error>(TokioIo::new(UnixStream::connect(path).await?))
        }))
        .await
    {
        Ok(channel) => TimerServiceClient::new(channel),
        Err(_) => {
            println!("ptimer is not running");
            process::exit(0)
        }
    }
}
