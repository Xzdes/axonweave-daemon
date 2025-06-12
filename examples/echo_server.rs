// src/bin/echo_server.rs

use tonic::{transport::Server, Request, Response, Status};

// Подключаемся к нашей общей библиотеке, чтобы получить доступ к сгенерированному коду
use axonweave_daemon::grpc_generated::echo_v1 as echo_api;

// Импортируем трейт и сервер для эхо-сервиса
use echo_api::echo_service_server::{EchoService, EchoServiceServer};
// Импортируем структуры запроса и ответа
use echo_api::{EchoRequest, EchoResponse};

// Определяем структуру нашего эхо-сервиса
#[derive(Debug, Default)]
pub struct MyEchoService {}

// Реализуем для нее трейт `EchoService`
#[tonic::async_trait]
impl EchoService for MyEchoService {
    // Реализуем наш единственный метод `say_hello`
    async fn say_hello(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let name = &request.into_inner().name;
        
        println!("[EchoServer] Received a 'SayHello' request with name: '{}'", name);

        // Создаем ответ
        let message = format!("Hello, {}! This is EchoServer.", name);
        let reply = EchoResponse { message };
        
        // Отправляем ответ
        Ok(Response::new(reply))
    }
}

// Главная функция для запуска эхо-сервиса
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Определяем адрес, на котором будет работать наш эхо-сервис.
    // Важно, чтобы он был ДРУГИМ, нежели у моста Axonweave.
    let addr = "127.0.0.1:50052".parse()?;
    
    // Создаем экземпляр нашего сервиса
    let echo_service = MyEchoService::default();

    println!("[EchoServer] Starting and listening on {}", addr);

    // Запускаем gRPC-сервер
    Server::builder()
        .add_service(EchoServiceServer::new(echo_service))
        .serve(addr)
        .await?;

    Ok(())
}