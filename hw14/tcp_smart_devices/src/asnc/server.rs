use crate::{decode_request, encode_response, Command, Request, Response};
use smart_devices::device::SmartSocket;
use std::sync::Arc;
use stp::asnc::server::{StpConnection, StpServer};
use tokio::sync::RwLock;

pub struct AsyncTcpSmartSocket {
    inner: Arc<RwLock<SmartSocket>>,
}

impl AsyncTcpSmartSocket {
    pub fn new(name: &str, description: &str, is_on: bool, current_power: f64) -> Self {
        let socket = SmartSocket::new(name, description, is_on, current_power);

        Self {
            inner: Arc::new(RwLock::new(socket)),
        }
    }
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
        serve(self.inner.clone(), addr)
    }
}

async fn serve(
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

async fn handle_request(socket: Arc<RwLock<SmartSocket>>, request: Request) -> Response {
    Response(match request.0 {
        Command::SmartSocketOn => {
            socket.clone().write().await.turn_on();
            format!("{}", socket.clone().read().await)
        }
        Command::SmartSocketOff => {
            socket.clone().write().await.turn_off();
            format!("{}", socket.clone().read().await)
        }
        Command::SmartSocketInfo => {
            format!("{}", socket.clone().read().await)
        }
        Command::SmartSocketState => {
            if socket.clone().read().await.is_on() {
                "on".to_owned()
            } else {
                "off".to_owned()
            }
        }
    })
}

async fn handle_connection(socket: Arc<RwLock<SmartSocket>>, mut connection: StpConnection) {
    loop {
        let socket = socket.clone();
        let process_result = connection
            .process_request_async(|req| async move {
                match decode_request(&req) {
                    Some(request) => encode_response(handle_request(socket.clone(), request).await),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn serve_turn_on() {
        let tcp_smart_socket = AsyncTcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            false,
            220.0,
        );

        let result = handle_request(
            tcp_smart_socket.inner.clone(),
            Request(Command::SmartSocketOn),
        )
        .await;

        assert_eq!(
            Response(
                r#"Name: tcp_smart_socket
Description: this is smart socket works by tcp protocol
Current state: on, 220 Volts"#
                    .to_owned()
            ),
            result
        );
    }

    #[tokio::test]
    async fn serve_turn_off() {
        let tcp_smart_socket = AsyncTcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            true,
            230.0,
        );

        let result = handle_request(
            tcp_smart_socket.inner.clone(),
            Request(Command::SmartSocketOff),
        )
        .await;

        assert_eq!(
            Response(
                r#"Name: tcp_smart_socket
Description: this is smart socket works by tcp protocol
Current state: off, 230 Volts"#
                    .to_owned()
            ),
            result
        );
    }

    #[tokio::test]
    async fn serve_info() {
        let tcp_smart_socket = AsyncTcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            true,
            233.3,
        );

        let result = handle_request(
            tcp_smart_socket.inner.clone(),
            Request(Command::SmartSocketInfo),
        )
        .await;

        assert_eq!(
            Response(
                r#"Name: tcp_smart_socket
Description: this is smart socket works by tcp protocol
Current state: on, 233.3 Volts"#
                    .to_owned()
            ),
            result
        );
    }

    #[tokio::test]
    async fn serve_state() {
        let tcp_smart_socket = AsyncTcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            true,
            233.3,
        );

        let result = handle_request(
            tcp_smart_socket.inner.clone(),
            Request(Command::SmartSocketState),
        )
        .await;

        assert_eq!(Response("on".to_owned()), result);

        _ = handle_request(
            tcp_smart_socket.inner.clone(),
            Request(Command::SmartSocketOff),
        )
        .await;

        let result = handle_request(
            tcp_smart_socket.inner.clone(),
            Request(Command::SmartSocketState),
        )
        .await;

        assert_eq!(Response("off".to_owned()), result);
    }
}
