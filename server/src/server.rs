use tonic::{transport::Server, Request, Response, Status};
use greet::greet_server::{Greet, GreetServer};
use greet::{GreetResponse, GreetRequest};

mod greet {
    tonic::include_proto!("greet");
}
#[derive(Default)]
pub struct MyGreet {}

#[tonic::async_trait]
impl Greet for MyGreet{
    async fn send(&self,request:Request<GreetRequest>)->Result<Response<GreetResponse>,Status>{
        Ok(Response::new(GreetResponse{
             message:format!("hey {}!",request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let say = MyGreet::default();
    println!("Server listening on {}", addr);
    
    Server::builder()
        .add_service(GreetServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}