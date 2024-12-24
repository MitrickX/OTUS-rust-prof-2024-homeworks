pub mod info;

use std::fmt;

pub struct SmartSocket {
    name: String,
    description: String,
    is_on: bool,
    current_power: f64,
}

impl SmartSocket {
    pub fn new(name: &str, description: &str, is_on: bool, current_power: f64) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            is_on,
            current_power,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn turn_on(&mut self) {
        self.is_on = true
    }

    pub fn turn_off(&mut self) {
        self.is_on = false
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn current_power(&self) -> f64 {
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
            "Current state: {}, {} Volts",
            if self.is_on { "on" } else { "off" },
            self.current_power
        )?;
        Ok(())
    }
}
pub struct SmartThermometer {
    name: String,
    description: String,
    current_temperature: f64,
}

impl SmartThermometer {
    pub fn new(name: &str, description: &str, current_temperature: f64) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            current_temperature,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn current_temperature(&self) -> f64 {
        self.current_temperature
    }

    pub fn set_temperature(&mut self, val: f64) {
        self.current_temperature = val
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("")?;
        writeln!(f, "Name: {}", &self.name)?;
        f.pad("")?;
        writeln!(f, "Description: {}", &self.description)?;
        f.pad("")?;
        write!(
            f,
            "Current temperature: {} Celsus",
            &self.current_temperature
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_smart_socket {
    use super::*;

    #[test]
    fn test_new() {
        let socket1 = SmartSocket::new("socket1", "description_socket1", true, 220.0);
        let socket2 = SmartSocket::new("socket2", "description_socket2", false, 0.0);

        assert_eq!(socket1.name(), "socket1");
        assert_eq!(socket1.description(), "description_socket1");
        assert!(socket1.is_on());
        assert_eq!(socket1.current_power(), 220.0);

        assert_eq!(socket2.name(), "socket2");
        assert_eq!(socket2.description(), "description_socket2");
        assert!(!socket2.is_on());
        assert_eq!(socket2.current_power(), 0.0);
    }

    #[test]
    fn test_turn_of_off() {
        let mut socket = SmartSocket::new("socket", "description_socket", true, 220.0);
        assert!(socket.is_on());
        socket.turn_off();
        assert!(!socket.is_on());
        socket.turn_on();
        assert!(socket.is_on());
    }
}

#[cfg(test)]
mod tests_smart_thermometr {
    use super::*;

    #[test]
    fn test_new() {
        let thermo = SmartThermometer::new("thermo", "thermo_description", 32.0);

        assert_eq!(thermo.name(), "thermo");
        assert_eq!(thermo.description(), "thermo_description");
        assert_eq!(thermo.current_temperature(), 32.0);
    }

    #[test]
    fn test_set_temperature() {
        let mut thermo = SmartThermometer::new("thermo", "thermo_description", 32.0);

        thermo.set_temperature(20.2);
        assert_eq!(thermo.current_temperature(), 20.2);
    }
}
