use super::{SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn info(&self, location_name: &str, device_name: &str) -> Option<String>;
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl OwningDeviceInfoProvider {
    pub fn new(socket: SmartSocket) -> Self {
        Self { socket }
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn info(&self, location_name: &str, device_name: &str) -> Option<String> {
        if self.socket.name() != device_name {
            return None;
        }

        Some(format!(
            r#"Location: {}
Device/Socket: 
{:<2}"#,
            location_name, self.socket,
        ))
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
    pub fn new(socket: &'a SmartSocket, thermo: &'b SmartThermometer) -> Self {
        Self { socket, thermo }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn info(&self, location_name: &str, device_name: &str) -> Option<String> {
        if self.socket.name() == device_name {
            return Some(format!(
                r#"Location: {}
Device/Socket: 
{:<2}"#,
                location_name, self.socket
            ));
        }

        if self.thermo.name() == device_name {
            return Some(format!(
                r#"Location: {}
Device/Thermometer: 
{:<2}"#,
                location_name, self.thermo
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owning_provider() {
        let socket = SmartSocket::new("test_socket_name", "test socket description", true, 220.2);

        let info_provider = OwningDeviceInfoProvider::new(socket);

        let info1 = info_provider
            .info("test_location_name", "test_socket_name")
            .unwrap();

        assert_eq!(
            r#"Location: test_location_name
Device/Socket: 
  Name: test_socket_name
  Description: test socket description
  Current state: on, 220.2 Volts"#,
            info1
        );

        let info2 = info_provider.info("test_location_name", "unknown_device_name");
        assert_eq!(None, info2)
    }

    #[test]
    fn test_borrowing_provider() {
        let socket = SmartSocket::new("test_socket_name", "test socket description", true, 220.2);
        let thermo = SmartThermometer::new("test_thermo_name", "test thermo description", 13.0);

        let info_provider = BorrowingDeviceInfoProvider::new(&socket, &thermo);

        let info1 = info_provider
            .info("test_location_name", "test_socket_name")
            .unwrap();

        assert_eq!(
            r#"Location: test_location_name
Device/Socket: 
  Name: test_socket_name
  Description: test socket description
  Current state: on, 220.2 Volts"#,
            info1
        );

        let info2 = info_provider
            .info("test_location_name", "test_thermo_name")
            .unwrap();

        assert_eq!(
            r#"Location: test_location_name
Device/Thermometer: 
  Name: test_thermo_name
  Description: test thermo description
  Current temperature: 13 Celsus"#,
            info2
        );

        let info3 = info_provider.info("test_location_name", "unknown_device_name");
        assert_eq!(None, info3);
    }
}
