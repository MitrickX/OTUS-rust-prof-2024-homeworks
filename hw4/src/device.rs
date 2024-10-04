use std::fmt;

#[derive(PartialEq, Debug)]
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

    fn description(&self) -> &str {
        &self.description
    }

    fn turn_on(&mut self) {
        self.is_on = true
    }

    fn turn_off(&mut self) {
        self.is_on = false
    }

    fn current_power(&self) -> f64 {
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
pub struct SmartThermometer {
    name: String,
    current_temperature: f64,
}

impl SmartThermometer {
    pub fn new(name: &str, current_temperature: f64) -> Self {
        Self {
            name: name.to_owned(),
            current_temperature,
        }
    }

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

#[cfg(test)]
mod tests_smart_socket {
    use super::*;

    #[test]
    fn test_new() {
        let socket1 = SmartSocket::new("socket1", "description_socket1", true, 220.0);
        let socket2 = SmartSocket::new("socket2", "description_socket2", false, 0.0);

        assert_eq!(
            socket1,
            SmartSocket {
                name: "socket1".to_owned(),
                description: "description_socket1".to_owned(),
                is_on: true,
                current_power: 220.0,
            }
        );

        assert_eq!(
            socket2,
            SmartSocket {
                name: "socket2".to_owned(),
                description: "description_socket2".to_owned(),
                is_on: false,
                current_power: 0.0,
            }
        );
    }
}
