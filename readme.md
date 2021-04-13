# gRPC with Rust

The cargo.toml file on the root contains the list of binaries, which are client and server.
To run both client and server, in different terminals execute the following commands:

```
cargo run --bin server
cargo run --bin client
```

I'm using [Tonic](https://github.com/hyperium/tonic), a Rust implementation of gRPC.

In this example, server and client are together in the same folder but, of course, in real life applications, they could likely be in separate folders and even be written in different languages., comunicating with each other via gRPC.

the `proto` folders contain the `protobuf` files, which contain the RPC specifications.
After building the project y running `cargo build`, the `target` folder will be created. This folder contain the structs and traits defined by the proto files.

The following example, a cut from the server.rs file, show the MyGreet struct, an implementation of the Greet trait, defined in the proto file.

```
#[derive(Default)]
pub struct MyGreet {}

#[tonic::async_trait]
impl Greet for MyGreet{
    async fn send(&self,request:Request<GreetRequest>)->Result<Response<GreetResponse>,Status>{
        Ok(Response::new(GreetResponse{
             message:format!("hello {}",request.get_ref().name),
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
```
Another important part is the inclusion of the greet mod, shown bellow.

```
mod greet {
    tonic::include_proto!("greet");
}
```

It needs to be included to import the structs and traits that were created according to the greet.proto file.