use std::{thread, time::Duration};

use udp_smart_devices::UdpSmartThermometer;

const ADDR: &str = "127.0.0.1:55331";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dur = Duration::from_millis(500);
    let thermometer = UdpSmartThermometer::new("test smart thermometer", "test description", 20.2);
    thermometer.run(ADDR, dur)?;

    loop {
        let temperature = thermometer.current_temperature();
        println!("current temperature is {}", temperature);

        thread::sleep(dur * 2);
    }
}
