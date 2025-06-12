// src/bin/proxy_test_client.rs

use prost::Message;
use tonic::Request;

use axonweave_daemon::grpc_generated::{
    axon_control_v1 as axon_control,
    echo_v1 as echo_api,
};

use axon_control::registry_client::RegistryClient;
// ИСПРАВЛЕНИЕ: Импортируем правильные типы
use axon_control::{RegistrationRequest, UnaryProxyRequest};
use echo_api::{EchoRequest, EchoResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Шаг 1: Подключаемся к мосту Axonweave ---
    let axon_addr = "http://127.0.0.1:50051";
    println!("[ProxyClient] Connecting to Axonweave at {}...", axon_addr);
    let mut axon_client = RegistryClient::connect(axon_addr).await?;
    println!("[ProxyClient] Connected!");

    // --- Шаг 2: Регистрируем наш эхо-сервис в мосте ---
    let echo_service_name = "echo.v1.EchoService";
    let echo_service_addr = "http://127.0.0.1:50052";
    
    println!("[ProxyClient] Registering '{}' at address '{}'...", echo_service_name, echo_service_addr);

    let reg_request = Request::new(RegistrationRequest {
        service_name: echo_service_name.to_string(),
        service_address: echo_service_addr.to_string(),
        security_key: "secret".to_string(),
    });
    axon_client.register_service(reg_request).await?;
    println!("[ProxyClient] Registration successful!");
    
    // --- Шаг 3: Готовимся к унарному прокси-вызову ---
    println!("\n[ProxyClient] Preparing to call 'SayHello' via Axonweave UnaryProxy...");

    let echo_request_payload = EchoRequest { name: "ProxiedUser".to_string() };
    
    let mut request_body_bytes = Vec::new();
    echo_request_payload.encode(&mut request_body_bytes)?;

    let proxy_request_payload = UnaryProxyRequest {
        path: "/echo.v1.EchoService/SayHello".to_string(),
        body: request_body_bytes.into(),
    };

    // --- Шаг 4: Выполняем унарный прокси-вызов! ---
    println!("[ProxyClient] Executing UnaryProxy to target: {}", &proxy_request_payload.path);
    // ИСПРАВЛЕНИЕ: Вызываем правильный метод `unary_proxy`
    let response = axon_client.unary_proxy(Request::new(proxy_request_payload)).await?;
    
    // --- Шаг 5: Обрабатываем ответ ---
    let proxy_response = response.into_inner();
    let response_body_bytes = proxy_response.body;
    
    // gRPC добавляет 5-байтовый префикс. Для унарных вызовов через универсальный клиент
    // этот префикс может быть, а может и не быть. Попробуем декодировать напрямую.
    // Если будет ошибка, вернемся к `&response_body_bytes[5..]`.
    let echo_response_payload = EchoResponse::decode(&response_body_bytes[..])?;

    println!("\n[ProxyClient] SUCCESS! Received proxied response:");
    println!("{:#?}", echo_response_payload);

    Ok(())
}