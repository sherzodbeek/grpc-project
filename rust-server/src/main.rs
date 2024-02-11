use tonic::{transport::Server, Request, Response, Status};

use server::hello_server::{Hello, HelloServer};
use server::{HelloMessageRequest, HelloMessageReponse};

pub mod server {
    tonic::include_proto!("message");
}

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn say_hello(
        &self,
        request: Request<HelloMessageRequest>,
    ) -> Result<Response<HelloMessageReponse>, Status> {
      let r = request.into_inner();
      println!("{}", r.message);
      Ok(Response::new(HelloMessageReponse { message: { 
          format!("Hello from Rust").into()
        }}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = HelloService::default();

    Server::builder()
        .add_service(HelloServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}