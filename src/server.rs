use tonic::{transport::Server, Request, Response, Status};

use pingpong::ping_pong_server::{PingPong, PingPongServer};
use pingpong::{PingPongRequest, PingPongResponse};

pub mod pingpong {
    tonic::include_proto!("pingpong");
}

#[derive(Debug, Default)]
pub struct PingPongService {}

#[tonic::async_trait]
impl PingPong for PingPongService {
    async fn send_ping(
        &self,
        request: Request<PingPongRequest>,
    ) -> Result<Response<PingPongResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = PingPongResponse {
            pong: "pong".to_owned(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8009".parse()?;
    let pingpong_service = PingPongService::default();
    Server::builder()
        .add_service(PingPongServer::new(pingpong_service))
        .serve(addr)
        .await?;
    Ok(())
}
