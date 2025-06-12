// src/main.rs

// ... (все до `impl Registry` без изменений)
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use prost::Message;

use tonic::transport::{Endpoint, Server};
use tonic::{Request, Response, Status};

use axonweave_daemon::grpc_generated::echo_v1::echo_service_client::EchoServiceClient;
use axonweave_daemon::grpc_generated::echo_v1::EchoRequest;

use axonweave_daemon::grpc_generated::axon_control_v1 as axon_control;
use axon_control::registry_server::{Registry, RegistryServer};
use axon_control::{
    UnaryProxyRequest, UnaryProxyResponse,
    HeartbeatRequest, HeartbeatResponse, RegistrationRequest, RegistrationResponse,
    UnregistrationRequest, UnregistrationResponse,
};

// ... (структуры ServiceInfo, ServiceRegistry, AxonweaveService и методы register/unregister/heartbeat) ...
#[derive(Debug, Clone)]
struct ServiceInfo { address: String, last_heartbeat: Instant, }
type ServiceRegistry = Arc<RwLock<HashMap<String, ServiceInfo>>>;
#[derive(Debug, Clone)]
pub struct AxonweaveService { registry: ServiceRegistry, }
impl AxonweaveService { fn new(registry: ServiceRegistry) -> Self { Self { registry } } }
#[tonic::async_trait]
impl Registry for AxonweaveService {
    async fn register_service(&self, req: Request<RegistrationRequest>) -> Result<Response<RegistrationResponse>, Status> {
        let request_data = req.into_inner();
        println!("[Registry] Registering: '{}' at address '{}'", request_data.service_name, request_data.service_address);
        let mut registry_map = self.registry.write().await;
        if registry_map.contains_key(&request_data.service_name) {
            return Err(Status::already_exists(format!("Service name '{}' is already taken", request_data.service_name)));
        }
        let info = ServiceInfo { address: request_data.service_address, last_heartbeat: Instant::now() };
        registry_map.insert(request_data.service_name.clone(), info);
        let reply = RegistrationResponse { status: 1, message: "OK".into(), heartbeat_interval_seconds: 15 };
        Ok(Response::new(reply))
    }
    async fn unregister_service(&self, _req: Request<UnregistrationRequest>) -> Result<Response<UnregistrationResponse>, Status> { todo!() }
    async fn heartbeat(&self, _req: Request<HeartbeatRequest>) -> Result<Response<HeartbeatResponse>, Status> { todo!() }

    async fn unary_proxy(&self, request: Request<UnaryProxyRequest>) -> Result<Response<UnaryProxyResponse>, Status> {
        let proxy_request = request.into_inner();
        let path_str = &proxy_request.path;
        let service_name = path_str.split('/').nth(1)
            .ok_or_else(|| Status::invalid_argument("Malformed proxy path"))?;
        println!("[Proxy] Got a UnaryProxy call for path: {}", path_str);

        let service_address = {
            let registry_map = self.registry.read().await;
            registry_map.get(service_name).map(|info| info.address.clone())
        }.ok_or_else(|| Status::not_found(format!("Service '{}' not found", service_name)))?;
        println!("[Proxy] Connecting to target service '{}' at {}", service_name, service_address);

        let client_channel = Endpoint::from_shared(service_address)
            .map_err(|e| Status::internal(format!("Invalid endpoint URI: {}", e)))?
            .connect().await.map_err(|e| Status::unavailable(format!("Failed to connect to target: {}", e)))?;

        // ИСПРАВЛЕНИЕ: Преобразуем &String в &str с помощью .as_str()
        let response_body = match path_str.as_str() {
            "/echo.v1.EchoService/SayHello" => {
                let mut echo_client = EchoServiceClient::new(client_channel);
                let echo_request = EchoRequest::decode(&proxy_request.body[..])
                    .map_err(|e| Status::invalid_argument(format!("Failed to decode request body: {}", e)))?;
                let response = echo_client.say_hello(Request::new(echo_request)).await?;
                let mut buf = Vec::new();
                response.into_inner().encode(&mut buf).unwrap();
                buf
            }
            _ => {
                return Err(Status::unimplemented(format!("Proxying for path '{}' is not implemented", path_str)));
            }
        };

        let reply = UnaryProxyResponse { body: response_body.into() };
        Ok(Response::new(reply))
    }
}


// ... (reaper_task и main без изменений) ...
async fn reaper_task(registry: ServiceRegistry, reap_interval: Duration, service_ttl: Duration) {
    println!("[Reaper] Task started. Checking every {:?} for services older than {:?}.", reap_interval, service_ttl);
    loop {
        tokio::time::sleep(reap_interval).await;
        // ...
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Axonweave] Starting daemon...");
    let service_registry = Arc::new(RwLock::new(HashMap::new()));
    let reaper_registry_clone = Arc::clone(&service_registry);
    tokio::spawn(async move {
        reaper_task(reaper_registry_clone, Duration::from_secs(10), Duration::from_secs(30)).await;
    });
    let service = AxonweaveService::new(Arc::clone(&service_registry));
    println!("[Axonweave] Service configured.");
    let addr = "127.0.0.1:50051".parse()?;
    println!("[Axonweave] Listening on TCP socket: {}", addr);
    Server::builder()
        .add_service(RegistryServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}