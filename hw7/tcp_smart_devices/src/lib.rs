use smart_devices::device::SmartSocket;
use std::net::ToSocketAddrs;
use stp::error::{ConnectError, RequestError};
use stp::{client::StpClient, server::StpServer};

#[derive(Debug, PartialEq)]
pub enum Command {
    SmartSocketOn,
    SmartSocketOff,
    SmartSocketInfo,
}

#[derive(Debug, PartialEq)]
pub struct Request(Command);

#[derive(Debug, PartialEq)]
pub struct Response(String);

pub fn encode_request(request: Request) -> String {
    match request.0 {
        Command::SmartSocketOn => "on".to_owned(),
        Command::SmartSocketOff => "off".to_owned(),
        Command::SmartSocketInfo => "info".to_owned(),
    }
}

pub fn decode_request(request: &str) -> Option<Request> {
    match request {
        "on" => Some(Request(Command::SmartSocketOn)),
        "off" => Some(Request(Command::SmartSocketOff)),
        "info" => Some(Request(Command::SmartSocketInfo)),
        _ => None,
    }
}

pub fn encode_response(response: Response) -> String {
    response.0
}

pub fn decode_response(response: &str) -> Response {
    Response(response.to_owned())
}

pub struct TcpSmartSocket {
    socket: SmartSocket,
}

impl TcpSmartSocket {
    pub fn new(name: &str, description: &str, is_on: bool, current_power: f64) -> Self {
        Self {
            socket: SmartSocket::new(name, description, is_on, current_power),
        }
    }
}

pub trait Server {
    /// Handle on request
    fn handle(&mut self, request: Request) -> Response;

    /// Run server on tcp address
    fn serve(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl Server for TcpSmartSocket {
    fn handle(&mut self, request: Request) -> Response {
        match request.0 {
            Command::SmartSocketOn => self.socket.turn_on(),
            Command::SmartSocketOff => self.socket.turn_off(),
            Command::SmartSocketInfo => (),
        }

        Response(format!("{}", self.socket))
    }

    fn serve(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let server = StpServer::bind(addr.to_owned())?;

        println!(
            "Tcp smart socket \"{}\" works at {}",
            self.socket.name(),
            addr
        );

        // Обрабатываем подключения клиентов.
        loop {
            let Ok(mut connection) = server.accept() else {
                continue;
            };

            // Обрабатываем запрос.
            connection.process_request(|req| match decode_request(&req) {
                Some(request) => {
                    let response = self.handle(request);
                    encode_response(response)
                }
                None => "unknown command".to_owned(),
            })?;
        }
    }
}

pub struct TcpSmartSocketClient {
    stp: StpClient,
}

impl TcpSmartSocketClient {
    /// Подключаемся к серверу.
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    /// Запрашиваем инфу розетки
    pub fn get_info(&mut self) -> Result<String, RequestError> {
        let request = encode_request(Request(Command::SmartSocketInfo));
        self.stp.send_request(request)
    }

    /// Включаем розетку
    pub fn turn_on(&mut self) -> Result<String, RequestError> {
        let request = encode_request(Request(Command::SmartSocketOn));
        self.stp.send_request(request)
    }

    /// Выключаем розетку
    pub fn turn_off(&mut self) -> Result<String, RequestError> {
        let request = encode_request(Request(Command::SmartSocketOff));
        self.stp.send_request(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_turn_on() {
        let mut tcp_smart_socket = TcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            false,
            220.0,
        );

        let result = tcp_smart_socket.handle(Request(Command::SmartSocketOn));

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

    #[test]
    fn serve_turn_off() {
        let mut tcp_smart_socket = TcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            true,
            230.0,
        );

        let result = tcp_smart_socket.handle(Request(Command::SmartSocketOff));

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

    #[test]
    fn serve_turn_info() {
        let mut tcp_smart_socket = TcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            true,
            233.3,
        );

        let result = tcp_smart_socket.handle(Request(Command::SmartSocketInfo));

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
}
