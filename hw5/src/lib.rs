pub mod device;

use device::info::DeviceInfoProvider;
use std::collections::HashMap;

pub struct SmartHouse {
    _name: String,
    _room_names: Vec<String>,
    devices: HashMap<String, Vec<String>>,
}

impl SmartHouse {
    pub fn new(name: &str, devices: HashMap<&str, Vec<&str>>) -> Self {
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

    pub fn create_report<I: DeviceInfoProvider>(&self, info_provider: &I) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInfoProvider {}

    impl DeviceInfoProvider for TestInfoProvider {
        fn info(&self, location_name: &str, device_name: &str) -> Option<String> {
            Some(format!(
                "location: {}, device: {}",
                location_name, device_name
            ))
        }
    }

    #[test]
    fn test_create_report() {
        let house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        let info_provider = TestInfoProvider {};

        let report = house.create_report(&info_provider);
        let mut report_lines = report.split("\n").collect::<Vec<&str>>();
        report_lines.sort();
        let report = report_lines.join("\n");

        assert_eq!(
            r#"location: room1, device: room1_socket_1
location: room1, device: room1_thermo_1
location: room2, device: room2_socket_2"#,
            report
        )
    }
}
