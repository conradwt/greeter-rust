use greeter_api::greeter_client::GreeterClient;
use greeter_api::HelloRequest;

pub mod greeter_api {
  tonic::include_proto!("greeterapi");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Conrad".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
