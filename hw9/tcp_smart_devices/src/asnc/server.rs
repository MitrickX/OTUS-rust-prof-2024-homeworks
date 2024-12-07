use crate::{decode_request, encode_response, Command, Request, Response};
use smart_devices::device::SmartSocket;
use std::sync::Arc;
use stp::asnc::server::{StpConnection, StpServer};
use tokio::sync::RwLock;

pub struct AsyncTcpSmartSocket {
    inner: Arc<RwLock<SmartSocket>>,
}

pub trait Server {
    /// Run server on tcp address
    fn serve(
        &self,
        addr: &str,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}

impl Server for AsyncTcpSmartSocket {
    fn serve(
        &self,
        addr: &str,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send {
        server(self.inner.clone(), addr)
    }
}

async fn server(
    socket: Arc<RwLock<SmartSocket>>,
    addr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = StpServer::bind(addr.to_owned()).await?;

    println!(
        "Tcp smart socket \"{}\" works at {}",
        socket.clone().read().await.name(),
        addr
    );

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(connection) = server.accept().await else {
            continue;
        };

        tokio::spawn(handle_connection(socket.clone(), connection));
    }
}

async fn handle_request(socket: Arc<RwLock<SmartSocket>>, request: Request) -> String {
    match request.0 {
        Command::SmartSocketOn => socket.clone().write().await.turn_on(),
        Command::SmartSocketOff => socket.clone().write().await.turn_off(),
        Command::SmartSocketInfo => (),
    }

    let response = Response(format!("{}", socket.clone().read().await));
    encode_response(response)
}

async fn handle_connection(socket: Arc<RwLock<SmartSocket>>, mut connection: StpConnection) {
    loop {
        let socket = socket.clone();
        let process_result = connection
            .process_request_async(|req| async move {
                match decode_request(&req) {
                    Some(request) => handle_request(socket.clone(), request).await,
                    None => "unknown command".to_owned(),
                }
            })
            .await;

        if let Err(e) = process_result {
            eprint!("Error processing request: {}", e);
            break;
        }
    }
}
