use std::io;
use thiserror::Error;

/// Ошибка соединения.
#[derive(Error, Debug)]
pub enum ConnectError {
    /// Неудачный handshake.
    #[error("bad handshake")]
    BadHandshake,

    /// Внутренняя ошибка IO.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Ошибка отправки сообщения.
#[derive(Error, Debug)]
pub enum SendError {
    /// Внутренняя ошибка IO.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Ошибка приема сообщения.
#[derive(Error, Debug)]
pub enum RecvError {
    /// Некорректная кодировка принятой строки.
    #[error("bad encoding")]
    BadEncoding,

    /// Внутренняя ошибка IO.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Ошибка при обмене данными с сервером.
#[derive(Error, Debug)]
pub enum RequestError {
    /// Ошибка отправки.
    #[error("send error: {0}")]
    Send(#[from] SendError),

    /// Ошибка приема.
    #[error("recv error {0}")]
    Recv(#[from] RecvError),
}
