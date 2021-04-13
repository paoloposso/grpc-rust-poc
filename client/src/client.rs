use greet::greet_client::GreetClient;
use greet::GreetRequest;

mod greet {
    tonic::include_proto!("greet");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;
    
    let mut client = GreetClient::new(channel);

    let request = tonic::Request::new(
        GreetRequest {
           name:String::from("Paolo")
        },
    );

    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
