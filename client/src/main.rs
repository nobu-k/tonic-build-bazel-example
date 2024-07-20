mod hello;
use hello::hello_service_client::HelloServiceClient;
use hello::HelloRequest;
use tokio_stream::StreamExt;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect("http://[::1]:50051").await?;

    let request_stream = async_stream::stream! {
        yield HelloRequest {
            name: "Alice".into(),
        };
        yield HelloRequest {
            name: "Bob".into(),
        };
    };

    let response = client.client_stream(request_stream).await?;
    println!("RESPONSE = {}", response.into_inner().message);

    let mut response = client
        .server_stream(HelloRequest {
            name: "Stream".into(),
        })
        .await?
        .into_inner();
    while let Some(res) = response.message().await? {
        println!("RESPONSE = {}", res.message);
    }

    Ok(())
}
