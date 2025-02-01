use tcp_smart_devices::asnc::server::{AsyncTcpSmartSocket, Server};

const ADDR: &str = "127.0.0.1:55331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_smart_socket = AsyncTcpSmartSocket::new(
        "Smarty electric",
        "this is smart socket works by tcp protocol",
        false,
        220.0,
    );

    tcp_smart_socket.serve(ADDR).await
}
