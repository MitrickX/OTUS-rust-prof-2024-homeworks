use smart_devices::device::SmartThermometer;
use std::future::Future;
use std::{
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::Mutex;
extern crate tokio;

type AMutex<T> = Arc<Mutex<T>>;

pub struct StreamingSmartThermometer {
    thermometer: AMutex<SmartThermometer>,
    finished: Arc<AtomicBool>,
}

impl StreamingSmartThermometer {
    pub fn new(name: &str, description: &str, current_temperature: f64) -> Self {
        Self {
            thermometer: Arc::new(Mutex::new(SmartThermometer::new(
                name,
                description,
                current_temperature,
            ))),
            finished: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn current_temperature(&self) -> f64 {
        self.thermometer.lock().await.current_temperature()
    }
}

pub trait Streaming {
    fn recv_from(
        &self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, std::io::Error>> + Send;
    fn set_timeout(&self, dur: Duration) -> std::io::Result<()>;
}

pub trait Server {
    fn run<S: Streaming + Send + 'static>(
        &self,
        reciever: S,
        dur: Duration,
    ) -> impl Future<Output = std::io::Result<()>> + Send;
}

impl Server for StreamingSmartThermometer {
    async fn run<S: Streaming + Send + 'static>(
        &self,
        streaming: S,
        dur: Duration,
    ) -> std::io::Result<()> {
        streaming.set_timeout(dur)?;

        let finished = self.finished.clone();
        let thermometer = self.thermometer.clone();
        tokio::spawn(async move {
            loop {
                if finished.load(Ordering::SeqCst) {
                    return;
                }

                tokio::time::sleep(dur).await;

                let mut buf = [0; 8];
                match streaming.recv_from(&mut buf).await {
                    Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(err) => {
                        println!("can't receive datagram: {err}");
                        continue;
                    }
                    Ok(_) => {}
                }
                let val = f64::from_be_bytes(buf);
                thermometer.lock().await.set_temperature(val);
            }
        });

        Ok(())
    }
}

impl Drop for StreamingSmartThermometer {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}

struct UpdStreaming(UdpSocket);

impl Streaming for UpdStreaming {
    async fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        let result = self.0.recv_from(buf)?;
        Ok(result.0)
    }
    fn set_timeout(&self, dur: Duration) -> std::io::Result<()> {
        self.0.set_read_timeout(Some(dur))
    }
}

pub struct UdpSmartThermometer(StreamingSmartThermometer);

impl UdpSmartThermometer {
    pub fn new(name: &str, description: &str, current_temperature: f64) -> Self {
        Self(StreamingSmartThermometer::new(
            name,
            description,
            current_temperature,
        ))
    }

    pub async fn current_temperature(&self) -> f64 {
        self.0.current_temperature().await
    }

    pub async fn run<A: ToSocketAddrs>(
        &self,
        address: A,
        dur: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(address)?;
        socket.set_nonblocking(false)?;
        let streaming = UpdStreaming(socket);
        self.0.run(streaming, dur).await?;
        Ok(())
    }
}

pub struct UdpSmartThermometerClient<A: ToSocketAddrs> {
    socket: UdpSocket,
    reciever_address: A,
}

impl<A: ToSocketAddrs> UdpSmartThermometerClient<A> {
    pub fn new(bind_address: A, reciever_address: A) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(bind_address)?;
        Ok(Self {
            socket,
            reciever_address,
        })
    }

    pub fn send_temperature(&self, val: f64) -> Result<(), Box<dyn std::error::Error>> {
        self.socket
            .send_to(val.to_be_bytes().as_slice(), &self.reciever_address)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};
    use tokio::sync::{
        mpsc::{self, Receiver},
        Mutex,
    };

    struct TestStreaming {
        receiver: Arc<Mutex<Receiver<[u8; 8]>>>,
    }

    impl Streaming for TestStreaming {
        async fn recv_from(&self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            thread::sleep(Duration::from_millis(10));
            let data = self.receiver.clone().lock().await.recv().await.unwrap();
            buf.copy_from_slice(&data);
            Ok(8)
        }
        fn set_timeout(&self, _dur: Duration) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_run() {
        let (tx, rx) = mpsc::channel(10);
        let streaming = TestStreaming {
            receiver: Arc::new(Mutex::new(rx)),
        };

        let thermo = StreamingSmartThermometer::new("test name", "test description", 32.0);

        let result = thermo.run(streaming, Duration::from_millis(10)).await;
        assert!(result.is_ok());

        assert_eq!(32.0, thermo.current_temperature().await);

        let result = tx.send(20.3f64.to_be_bytes()).await;
        assert!(result.is_ok());

        thread::sleep(Duration::from_millis(200));

        assert_eq!(20.3, thermo.current_temperature().await);

        let result = tx.send(11.5f64.to_be_bytes()).await;
        assert!(result.is_ok());

        thread::sleep(Duration::from_millis(200));

        assert_eq!(11.5, thermo.current_temperature().await);
    }
}
