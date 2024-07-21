use std::pin::Pin;

use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{transport::Server, Request, Response, Status};

use hello::{
    hello_service_server::{HelloService, HelloServiceServer},
    HelloRequest, HelloResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyHelloService::default();

    Server::builder()
        .add_service(HelloServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    type ServerStreamStream =
        Pin<Box<dyn Stream<Item = Result<HelloResponse, Status>> + Send + Sync + 'static>>;

    // TODO: check how to access Status.details
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let request = request.into_inner();
        let reply = HelloResponse {
            message: format!("Hello, {}!", request.name),
        };
        Ok(Response::new(reply))
    }

    async fn server_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::ServerStreamStream>, Status> {
        let request = request.into_inner();
        let stream = async_stream::stream! {
            yield Ok(HelloResponse {
                message: format!("Hello, {}!", request.name),
            });
            yield Ok(HelloResponse {
                message: format!("Hello, {}!", request.name),
            });
            yield Ok(HelloResponse {
                message: format!("Hello, {}!", request.name),
            });
        };
        Ok(Response::new(Box::pin(stream)))
    }

    async fn client_stream(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<HelloResponse>, Status> {
        let mut stream = request.into_inner();
        let mut names = vec![];
        while let Some(req) = stream.message().await? {
            names.push(req.name);
        }
        let reply = HelloResponse {
            message: format!("Hello, {}!", names.join(", ")),
        };
        println!("reply = {:?}", reply);
        Ok(Response::new(reply))
    }
}
