use std::error::Error;
use stp::server::StpServer;
use tcp_smart_devices::{decode_request, encode_response, ServeRequest, TcpSmartSocket};

fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr = String::from("127.0.0.1:55331");
    let server = StpServer::bind(addr)?;

    let mut tcp_smart_socket = TcpSmartSocket::new(
        "tcp_smart_socket",
        "this is smart socket works by tcp protocol",
        false,
        220.0,
    );

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(mut connection) = server.accept() else {
            continue;
        };

        // Обрабатываем запрос.
        connection.process_request(|req| match decode_request(&req) {
            Ok(request) => {
                let response = tcp_smart_socket.serve(request);
                match encode_response(response) {
                    Ok(result) => result,
                    Err(err) => format!("{}", err),
                }
            }
            Err(err) => format!("{}", err),
        })?;
    }
}
