use hw4::{
    device::{SmartSocket, SmartThermometer},
    info::BorrowingDeviceInfoProvider,
    info::OwningDeviceInfoProvider,
    SmartHouse,
};
use std::collections::HashMap;

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new(
        "room1_socket_1",
        "Smart Plug WiFi Socket EU 16A/20A With Power Monitor Timing Function Tuya Smart Life APP Control Works With Alexa Google Home",
        true,
        225.5,
    );

    let socket2 = SmartSocket::new(
        "room2_socket_2",
        "Smart Plug WiFi Socket EU 16A/20A With Power Monitor Timing Function Tuya Smart Life APP Control Works With Alexa Google Home",
        false,
        0.0,
    );

    let thermo = SmartThermometer::new("room1_thermo_1", "Govee WiFi Hygrometer Thermometer Sensor 3 Pack, Indoor Wireless Smart Temperature Humidity Monitor with Remote App Notification Alert, 2 Years Data Storage Export, for Home, Greenhouse", 19.2);

    // Инициализация дома
    let house = SmartHouse::new(
        "my smart house",
        HashMap::from([
            ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
            ("room2", vec!["room2_socket_2"]),
        ]),
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!(
        r#"
=== Report #1: ===

{}
"#,
        report1
    );

    println!(
        r#"
=== Report #2: ===

{}
"#,
        report2
    );
}
