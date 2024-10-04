use crate::device::{SmartSocket, SmartThermometer};

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

Devices (1):
Socket: 
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
        if self.socket.name() != device_name {
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
