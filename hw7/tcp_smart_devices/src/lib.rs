use serde::{Deserialize, Serialize};
use smart_devices::device::SmartSocket;

#[derive(Serialize, Deserialize)]
pub enum Command {
    SmartSocketOn,
    SmartSocketOff,
    SmartSocketInfo,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String, // name of device
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response {
    pub status: bool,
    pub message: String,
}

pub fn encode_request(request: Request) -> serde_json::Result<String> {
    serde_json::to_string(&request)
}

pub fn decode_request(request: &str) -> serde_json::Result<Request> {
    serde_json::from_str(request)
}

pub fn encode_response(response: Response) -> serde_json::Result<String> {
    serde_json::to_string(&response)
}

pub fn decode_response(response: &str) -> serde_json::Result<Response> {
    serde_json::from_str(response)
}

pub trait ServeRequest {
    fn serve(&mut self, request: Request) -> Response;
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

impl ServeRequest for TcpSmartSocket {
    fn serve(&mut self, request: Request) -> Response {
        if self.socket.name() != request.name {
            return Response {
                status: false,
                message: "name mismatch".to_owned(),
            };
        }

        match request.command {
            Command::SmartSocketOn => self.socket.turn_on(),
            Command::SmartSocketOff => self.socket.turn_off(),
            Command::SmartSocketInfo => (),
        }

        Response {
            status: true,
            message: format!("{}", self.socket).to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_name_mismatch() {
        let mut tcp_smart_socket = TcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            false,
            220.0,
        );

        let result = tcp_smart_socket.serve(Request {
            name: "test".to_owned(),
            command: Command::SmartSocketOn,
        });

        assert_eq!(
            Response {
                status: false,
                message: "name mismatch".to_owned()
            },
            result
        );
    }

    #[test]
    fn serve_turn_on() {
        let mut tcp_smart_socket = TcpSmartSocket::new(
            "tcp_smart_socket",
            "this is smart socket works by tcp protocol",
            false,
            220.0,
        );

        let result = tcp_smart_socket.serve(Request {
            name: "tcp_smart_socket".to_owned(),
            command: Command::SmartSocketOn,
        });

        assert_eq!(
            Response {
                status: true,
                message: r#"Name: tcp_smart_socket
Description: this is smart socket works by tcp protocol
Current state: on, 220 Volts"#
                    .to_owned(),
            },
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

        let result = tcp_smart_socket.serve(Request {
            name: "tcp_smart_socket".to_owned(),
            command: Command::SmartSocketOff,
        });

        assert_eq!(
            Response {
                status: true,
                message: r#"Name: tcp_smart_socket
Description: this is smart socket works by tcp protocol
Current state: off, 230 Volts"#
                    .to_owned(),
            },
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

        let result = tcp_smart_socket.serve(Request {
            name: "tcp_smart_socket".to_owned(),
            command: Command::SmartSocketInfo,
        });

        assert_eq!(
            Response {
                status: true,
                message: r#"Name: tcp_smart_socket
Description: this is smart socket works by tcp protocol
Current state: on, 233.3 Volts"#
                    .to_owned(),
            },
            result
        );
    }
}
