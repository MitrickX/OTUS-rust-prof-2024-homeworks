use std::{thread, time::Duration};

use udp_smart_devices::UdpSmartThermometerClient;

const RECIEVER_ADDR: &str = "127.0.0.1:55331";
const BIND_ADDR: &str = "127.0.0.1:55330";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UdpSmartThermometerClient::new(BIND_ADDR, RECIEVER_ADDR)?;

    let mut temperature = 10.4;
    for i in 0..300 {
        temperature += 0.2 * i as f64;
        match client.send_temperature(temperature) {
            Ok(_) => println!("successfully send temperature {}", temperature),
            Err(e) => println!("failure send temperature {}, cause of {}", temperature, e),
        };
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
