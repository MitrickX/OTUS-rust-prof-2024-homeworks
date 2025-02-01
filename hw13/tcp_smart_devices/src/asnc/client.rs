use crate::{encode_request, Command, Request};
use stp::asnc::client::StpClient;
use stp::error::{ConnectError, RequestError};
use tokio::net::ToSocketAddrs;

pub struct AsyncTcpSmartSocketClient {
    stp: StpClient,
}

impl AsyncTcpSmartSocketClient {
    /// Подключаемся к серверу.
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr).await?;
        Ok(Self { stp })
    }

    /// Запрашиваем инфу розетки
    pub async fn get_info(&mut self) -> Result<String, RequestError> {
        let request = encode_request(Request(Command::SmartSocketInfo));
        self.stp.send_request(request).await
    }

    /// Включаем розетку
    pub async fn turn_on(&mut self) -> Result<String, RequestError> {
        let request = encode_request(Request(Command::SmartSocketOn));
        self.stp.send_request(request).await
    }

    /// Выключаем розетку
    pub async fn turn_off(&mut self) -> Result<String, RequestError> {
        let request = encode_request(Request(Command::SmartSocketOff));
        self.stp.send_request(request).await
    }
}
