use smart_devices::device::SmartThermometer;
use std::{
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self},
    time::Duration,
};

pub mod asnc;

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

    pub fn current_temperature(&self) -> f64 {
        self.thermometer.lock().unwrap().current_temperature()
    }
}

pub trait Streaming {
    fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn set_timeout(&self, dur: Duration) -> std::io::Result<()>;
}

pub trait Server {
    fn run<S: Streaming + Send + 'static>(&self, reciever: S, dur: Duration)
        -> std::io::Result<()>;
}

impl Server for StreamingSmartThermometer {
    fn run<S: Streaming + Send + 'static>(
        &self,
        streaming: S,
        dur: Duration,
    ) -> std::io::Result<()> {
        streaming.set_timeout(dur)?;

        let finished = self.finished.clone();
        let thermometer = self.thermometer.clone();
        thread::spawn(move || loop {
            if finished.load(Ordering::SeqCst) {
                return;
            }

            let mut buf = [0; 8];
            match streaming.recv_from(&mut buf) {
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
            thermometer.lock().unwrap().set_temperature(val);
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
    fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<usize> {
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

    pub fn current_temperature(&self) -> f64 {
        self.0.current_temperature()
    }

    pub fn run<A: ToSocketAddrs>(
        &self,
        address: A,
        dur: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(address)?;
        socket.set_nonblocking(false)?;
        let streaming = UpdStreaming(socket);
        self.0.run(streaming, dur)?;
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
    use std::sync::mpsc::{self, Receiver};

    struct TestStreaming {
        receiver: Receiver<[u8; 8]>,
    }

    impl Streaming for TestStreaming {
        fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<usize> {
            let data = self.receiver.recv().unwrap();
            buf.copy_from_slice(&data);
            Ok(8)
        }
        fn set_timeout(&self, _dur: Duration) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_run() {
        let (tx, rx) = mpsc::channel();
        let streaming = TestStreaming { receiver: rx };

        let thermo = StreamingSmartThermometer::new("test name", "test description", 32.0);
        let result = thermo.run(streaming, Duration::from_secs(1));
        assert!(result.is_ok());

        assert_eq!(32.0, thermo.current_temperature());

        let result = tx.send(20.3f64.to_be_bytes());
        assert!(result.is_ok());

        thread::sleep(Duration::from_millis(100));

        assert_eq!(20.3, thermo.current_temperature());

        let result = tx.send(11.5f64.to_be_bytes());
        assert!(result.is_ok());

        thread::sleep(Duration::from_millis(100));

        assert_eq!(11.5, thermo.current_temperature());
    }
}
