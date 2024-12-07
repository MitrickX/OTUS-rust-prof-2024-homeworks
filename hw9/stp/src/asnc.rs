pub mod client;
pub mod server;

use crate::error::{RecvError, SendError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Отправляет четыре байта `data.len()`, а потом сами данные.
pub async fn send_string<D, W>(data: D, mut writer: W) -> Result<(), SendError>
where
    D: AsRef<str>,
    W: AsyncWriteExt + Unpin,
{
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    writer.flush().await?;
    Ok(())
}

/// Читает четыре байта длины, а потом сами данные.
pub async fn recv_string<R>(mut reader: R) -> Result<String, RecvError>
where
    R: AsyncReadExt + Unpin,
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);
    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

#[cfg(test)]
mod tests {
    use super::{recv_string, send_string};

    #[tokio::test]
    async fn test_send_recv() {
        let message = "Test message";
        let mut buf = Vec::new();

        send_string(message, &mut buf).await.unwrap();
        let result = recv_string(&buf[..]).await.unwrap();

        assert_eq!(message, result)
    }

    #[tokio::test]
    async fn test_send() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        send_string(&data, &mut buf).await.unwrap();

        let len = u32::from_be_bytes(buf[..4].try_into().unwrap());
        let string_data = String::from_utf8(buf[4..].to_vec()).unwrap();

        assert_eq!(data, string_data);
        assert_eq!(len, 5);
    }

    #[tokio::test]
    async fn test_recv() {
        let data = String::from("hello");
        let mut buf = Vec::new();
        buf.extend_from_slice(&5_u32.to_be_bytes());
        buf.extend_from_slice(data.as_bytes());

        let received = recv_string(&buf[..]).await.unwrap();
        assert_eq!(data, received);
    }
}
