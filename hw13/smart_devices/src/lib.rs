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
    name: String,
    room_names: Vec<String>,
    devices: HashMap<String, Vec<String>>,
}

impl SmartHouse {
    pub fn new_empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            room_names: Vec::new(),
            devices: HashMap::new(),
        }
    }
    /// Конструктор дома
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
            name: name.to_owned(),
            room_names,
            devices: house_devices,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Перечисляем комнаты
    pub fn rooms(&self) -> impl Iterator<Item = String> {
        self.room_names.clone().into_iter()
    }

    /// Добавляем новую комнату, если еще не сущ-т
    pub fn add_room(&mut self, room: &str) {
        let room = room.to_owned();
        if !self.room_names.contains(&room) {
            self.room_names.push(room.clone());
            self.devices.insert(room, Vec::new());
        }
    }

    /// Удаляем сущ-щую комнату
    pub fn delete_room(&mut self, room: &str) {
        self.room_names.retain(|r| r != room);
        self.devices.remove(room);
    }

    /// Перечисляем девайсы комнаты
    pub fn devices(&self, room: &str) -> impl Iterator<Item = String> {
        self.devices
            .get(room)
            .cloned()
            .unwrap_or_default()
            .into_iter()
    }

    /// Добавляем девайс в сущ-шую комнату
    /// Если комнаты не сущ-т, то и девайс не добавится
    pub fn add_device(&mut self, room: &str, device: &str) {
        self.devices
            .entry(room.to_owned())
            .and_modify(|devices| devices.push(device.to_owned()));
    }

    /// Удаляем девайс из комнаты, если девайс и комнтата сущ-т
    pub fn delete_device(&mut self, room: &str, device: &str) {
        self.devices
            .entry(room.to_owned())
            .and_modify(|devices| devices.retain(|d| d != device));
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

    #[test]
    fn test_get_rooms() {
        let house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        let mut rooms: Vec<String> = house.rooms().collect();
        rooms.sort();

        assert_eq!(vec!["room1", "room2"], rooms);
    }

    #[test]
    fn test_add_room() {
        let mut house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        house.add_room("room3");

        let mut rooms: Vec<String> = house.rooms().collect();
        rooms.sort();

        assert_eq!(vec!["room1", "room2", "room3"], rooms);
    }

    #[test]
    fn test_delete_room() {
        let mut house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        house.delete_room("room1");

        let rooms: Vec<String> = house.rooms().collect();
        assert_eq!(vec!["room2"], rooms);
    }

    #[test]
    fn test_get_devices() {
        let house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        let mut devices: Vec<String> = house.devices("room1").collect();
        devices.sort();

        assert_eq!(vec!["room1_socket_1", "room1_thermo_1"], devices);
    }

    #[test]
    fn test_add_device() {
        let mut house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                ("room1", vec!["room1_socket_1", "room1_thermo_1"]),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        house.add_device("room1", "room1_socket_3");
        let mut devices: Vec<String> = house.devices("room1").collect();
        devices.sort();

        assert_eq!(
            vec!["room1_socket_1", "room1_socket_3", "room1_thermo_1",],
            devices
        );

        house.add_device("room3", "room3_socket_1");

        let mut rooms: Vec<String> = house.rooms().collect();
        rooms.sort();
        assert_eq!(vec!["room1", "room2"], rooms);

        let devices: Vec<String> = house.devices("room3").collect();
        assert_eq!(Vec::<String>::new(), devices);
    }

    #[test]
    fn test_add_device_to_empty() {
        let mut house = SmartHouse::new_empty("my smart house");

        house.add_room("room1");
        house.add_device("room1", "device1");
        let mut devices: Vec<String> = house.devices("room1").collect();
        devices.sort();

        assert_eq!(vec!["device1"], devices);
    }

    #[test]
    fn test_delete_device() {
        let mut house = SmartHouse::new(
            "my smart house",
            HashMap::from([
                (
                    "room1",
                    vec!["room1_socket_1", "room1_thermo_1", "room1_socket_3"],
                ),
                ("room2", vec!["room2_socket_2"]),
            ]),
        );

        house.delete_device("room1", "room1_socket_1");
        house.delete_device("room2", "room2_socket_2");
        house.delete_device("room3", "some_unexisting_device");

        let mut devices: Vec<String> = house.devices("room1").collect();
        devices.sort();
        assert_eq!(vec!["room1_socket_3", "room1_thermo_1",], devices);

        let mut devices: Vec<String> = house.devices("room2").collect();
        devices.sort();
        assert_eq!(Vec::<String>::new(), devices);

        let mut devices: Vec<String> = house.devices("room3").collect();
        devices.sort();
        assert_eq!(Vec::<String>::new(), devices);

        let mut rooms: Vec<String> = house.rooms().collect();
        rooms.sort();
        assert_eq!(vec!["room1", "room2"], rooms);
    }
}
