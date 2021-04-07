use hello::say_client::SayClient;
use hello::SayRequest;

mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;
    
    let mut client = SayClient::new(channel);

    let request = tonic::Request::new(
        SayRequest {
           name:String::from("Paolo")
        },
    );

    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
