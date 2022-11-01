use pingpong::ping_pong_client::PingPongClient;
use pingpong::PingPongRequest;

pub mod pingpong {
    tonic::include_proto!("pingpong");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PingPongClient::connect("http://[::1]:8009").await?;
    let request = tonic::Request::new(PingPongRequest {
        ping: "ping".to_owned(),
    });
    let response = client.count(request).await?;
    println!("Response={:?}", response);
    Ok(())
}
