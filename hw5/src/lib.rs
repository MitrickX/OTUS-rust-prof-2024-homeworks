pub mod device;

use device::info::DeviceInfoProvider;
use std::{collections::HashMap, ops::ControlFlow};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SmartHouseError {
    #[error("create report error")]
    ReportError,
}

type Result<T> = std::result::Result<T, SmartHouseError>;

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

    fn create_room_report<I: DeviceInfoProvider>(
        &self,
        room_name: &str,
        room_devices: &[String],
        info_provider: &I,
    ) -> Result<String> {
        let device_reports = match room_devices
            .iter()
            .map(move |device_name| info_provider.info(room_name, device_name))
            .try_fold(Vec::<String>::new(), |mut acc, x| match x {
                Some(s) => {
                    acc.push(s);
                    ControlFlow::Continue(acc)
                }
                None => ControlFlow::Break(()),
            }) {
            ControlFlow::Continue(v) => v,
            ControlFlow::Break(_) => {
                return Err(SmartHouseError::ReportError);
            }
        };

        Ok(device_reports.join("\n"))
    }

    pub fn create_report<I: DeviceInfoProvider>(&self, info_provider: &I) -> Result<String> {
        let room_devices_reports = match self
            .devices
            .iter()
            .map(|(room_name, room_devices)| {
                self.create_room_report(room_name, room_devices, info_provider)
            })
            .try_fold(Vec::<String>::new(), |mut acc, x| match x {
                Ok(s) => {
                    acc.push(s);
                    ControlFlow::Continue(acc)
                }
                Err(err) => ControlFlow::Break(err),
            }) {
            ControlFlow::Break(err) => return Err(err),
            ControlFlow::Continue(v) => v,
        };

        Ok(room_devices_reports.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestOkInfoProvider {}

    impl DeviceInfoProvider for TestOkInfoProvider {
        fn info(&self, location_name: &str, device_name: &str) -> Option<String> {
            Some(format!(
                "location: {}, device: {}",
                location_name, device_name
            ))
        }
    }

    struct TestFailInfoProvider {}

    impl DeviceInfoProvider for TestFailInfoProvider {
        fn info(&self, _location_name: &str, _device_name: &str) -> Option<String> {
            None
        }
    }

    #[test]
    fn test_create_report_ok() {
        let house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        let info_provider = TestOkInfoProvider {};

        let report = house.create_report(&info_provider).unwrap();
        let mut report_lines = report.split('\n').collect::<Vec<&str>>();
        report_lines.sort();
        let report = report_lines.join("\n");

        assert_eq!(
            r#"location: room1, device: room1_socket_1
location: room1, device: room1_thermo_1
location: room2, device: room2_socket_2"#,
            report
        )
    }

    #[test]
    fn test_create_report_fail() {
        let house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        let info_provider = TestFailInfoProvider {};

        let report = house.create_report(&info_provider);
        assert_eq!(Err(SmartHouseError::ReportError), report)
    }
}
