use tcp_smart_devices::{Server, TcpSmartSocket};

const ADDR: &str = "127.0.0.1:55331";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tcp_smart_socket = TcpSmartSocket::new(
        "Smarty electric",
        "this is smart socket works by tcp protocol",
        false,
        220.0,
    );

    tcp_smart_socket.serve(ADDR)
}
