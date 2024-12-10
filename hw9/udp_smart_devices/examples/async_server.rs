use std::time::Duration;
use udp_smart_devices::asnc::UdpSmartThermometer;

const ADDR: &str = "127.0.0.1:55331";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dur = Duration::from_millis(500);
    let thermometer = UdpSmartThermometer::new("test smart thermometer", "test description", 20.2);
    thermometer.run(ADDR, dur).await?;

    loop {
        let temperature = thermometer.current_temperature().await;
        println!("current temperature is {}", temperature);

        tokio::time::sleep(dur).await;
    }
}
