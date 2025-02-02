use std::time::Duration;

use udp_smart_devices::asnc::UdpSmartThermometerClient;

const RECIEVER_ADDR: &str = "127.0.0.1:55331";
const BIND_ADDR: &str = "127.0.0.1:55330";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UdpSmartThermometerClient::new(BIND_ADDR, RECIEVER_ADDR).await?;

    let mut temperature = 10.4;
    for i in 0..300 {
        temperature += 0.2 * i as f64;
        match client.send_temperature(temperature).await {
            Ok(_) => println!("successfully send temperature {}", temperature),
            Err(e) => println!("failure send temperature {}, cause of {}", temperature, e),
        };
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
