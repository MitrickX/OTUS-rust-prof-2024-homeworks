use std::collections::HashMap;
use std::fmt;

struct SmartHouse {
    _name: String,
    _room_names: Vec<String>,
    devices: HashMap<String, Vec<String>>,
}

impl SmartHouse {
    fn new(name: &str, devices: HashMap<&str, Vec<&str>>) -> Self {
        let room_names = devices.keys().map(|n| (*n).to_owned()).collect();

        let house_devices = devices
            .iter()
            .map(|(room_name, room_devices)| {
                (
                    (*room_name).to_owned(),
                    (*room_devices).iter().map(|d| (*d).to_owned()).collect(),
                )
            })
            .collect();

        Self {
            _name: name.to_owned(),
            _room_names: room_names,
            devices: house_devices,
        }
    }

    fn _get_rooms(&self) -> Vec<String> {
        self._room_names.clone()
    }

    fn _devices(&self, room: &str) -> Vec<String> {
        self.devices.get(room).cloned().unwrap_or_default()
    }

    fn create_report<I: DeviceInfoProvider>(&self, info_provider: &I) -> String {
        self.devices
            .iter()
            .map(|(room_name, room_devices)| {
                room_devices
                    .iter()
                    .filter_map(move |device_name| info_provider.info(room_name, device_name))
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

// Пользовательские устройства:
struct SmartSocket {
    name: String,
    description: String,
    is_on: bool,
    current_power: f64,
}

impl SmartSocket {
    fn _name(&self) -> &str {
        &self.name
    }

    fn _description(&self) -> &str {
        &self.description
    }

    fn _turn_on(&mut self) {
        self.is_on = true
    }

    fn _turn_off(&mut self) {
        self.is_on = false
    }

    fn _current_power(&self) -> f64 {
        self.current_power
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("")?;
        writeln!(f, "Name: {}", &self.name)?;
        f.pad("")?;
        writeln!(f, "Description: {}", &self.description)?;
        f.pad("")?;
        write!(
            f,
            "Current state:: {}, {} Volts",
            if self.is_on { "on" } else { "off" },
            self.current_power
        )?;
        Ok(())
    }
}

struct SmartThermometer {
    name: String,
    current_temperature: f64,
}

impl SmartThermometer {
    fn _current_temperature(&self) -> f64 {
        self.current_temperature
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("")?;
        writeln!(f, "Name: {}", &self.name)?;
        write!(f, "Current temperature: {}", &self.current_temperature)?;
        Ok(())
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn info(&self, location_name: &str, device_name: &str) -> Option<String>;
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn info(&self, location_name: &str, device_name: &str) -> Option<String> {
        if self.socket.name != device_name {
            return None;
        }

        Some(format!(
            r#"Location: {}

Devices (1):
Socket: 
{:<2}"#,
            location_name, self.socket,
        ))
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn info(&self, location_name: &str, device_name: &str) -> Option<String> {
        if self.socket.name != device_name {
            return None;
        }

        Some(format!(
            r#"Location: {}

Devices (2): 
Socket: 
{:<2}
Thermo: 
{:<2}"#,
            location_name, self.socket, self.thermo
        ))
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        name: "room1_socket_1".to_owned(),
        description: "Smart Plug WiFi Socket EU 16A/20A With Power Monitor Timing Function Tuya Smart Life APP Control Works With Alexa Google Home".to_owned(),
        is_on: true,
        current_power: 225.5,
    };
    let socket2 = SmartSocket {
        name: "room2_socket_2".to_owned(),
        description: "Smart Plug WiFi Socket EU 16A/20A With Power Monitor Timing Function Tuya Smart Life APP Control Works With Alexa Google Home".to_owned(),
        is_on: false,
        current_power: 0.0,
    };

    let thermo = SmartThermometer {
        name: "room1_thermo_1".to_owned(),
        current_temperature: 19.2,
    };

    // Инициализация дома
    let house = SmartHouse::new(
        "my smart house",
        HashMap::from([
            ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
            ("room2", vec!["room2_socket_2"]),
        ]),
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
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
