use smart_devices::{
    device::{
        info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
        SmartSocket, SmartThermometer,
    },
    SmartHouse,
};
use std::collections::HashMap;

// Пример использования
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
    let house_1 = SmartHouse::new(
        "my smart house",
        HashMap::from([
            ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
            ("room2", vec!["room2_socket_2"]),
        ]),
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider::new(socket1);
    let report_1 = house_1.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    let report_2 = house_1.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!(
        r#"
=== Report #1: ===

{}
"#,
        match report_1 {
            Ok(r) => r,
            Err(err) => format!("{:?}", err),
        }
    );

    println!(
        r#"
=== Report #2: ===

{}
"#,
        match report_2 {
            Ok(r) => r,
            Err(err) => format!("{:?}", err),
        }
    );

    let mut house_2 = SmartHouse::new(
        "my smart house",
        HashMap::from([("room1", vec!["room1_thermo_1", "room1_socket_3"])]),
    );

    house_2.delete_device("room1", "room1_socket_3");
    house_2.add_room("room2");
    house_2.add_device("room2", "room2_socket_1");
    house_2.add_device("room2", "room2_socket_2");
    house_2.delete_device("room2", "room2_socket_1");

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_3 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);
    let report_3 = house_2.create_report(&info_provider_3);

    println!(
        r#"
=== Report #3: ===

{}
"#,
        match report_3 {
            Ok(r) => r,
            Err(err) => format!("{:?}", err),
        }
    );

    println!("DEBUG");
    println!("Rooms: {:?}", house_2.rooms().collect::<Vec<String>>());
    println!(
        "Devices of room1: {:?}",
        house_2.devices("room1").collect::<Vec<String>>()
    );
}
