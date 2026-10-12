// Make readings print as "hall: 21.5°C". Replace todo!(), then run the checks.
use std::fmt;

pub struct Reading {
    pub device: String,
    pub celsius: f64,
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reading(device: &str, celsius: f64) -> Reading {
        Reading { device: device.to_string(), celsius }
    }

    #[test]
    fn prints_device_and_temperature() {
        assert_eq!(format!("{}", reading("hall", 21.5)), "hall: 21.5°C");
    }

    #[test]
    fn rounds_to_one_decimal() {
        assert_eq!(reading("hall", 21.46).to_string(), "hall: 21.5°C");
    }

    #[test]
    fn whole_numbers_still_show_a_decimal() {
        assert_eq!(reading("attic", -3.0).to_string(), "attic: -3.0°C");
    }

    #[test]
    fn works_inside_a_longer_message() {
        assert_eq!(format!("[{}]", reading("garage", 9.0)), "[garage: 9.0°C]");
    }
}
