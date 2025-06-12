// src/bin/direct_echo_client.rs

use axonweave_daemon::grpc_generated::echo_v1 as echo_api;
use echo_api::echo_service_client::EchoServiceClient;
use echo_api::EchoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Подключаемся НАПРЯМУЮ к эхо-серверу
    let mut client = EchoServiceClient::connect("http://127.0.0.1:50052").await?;
    
    let request = tonic::Request::new(EchoRequest {
        name: "DirectCaller".to_string(),
    });

    println!("[DirectClient] Sending a direct 'SayHello' request...");
    let response = client.say_hello(request).await?;

    println!("[DirectClient] Received response: {:#?}", response.into_inner());
    Ok(())
}