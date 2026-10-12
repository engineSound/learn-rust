// Parse a device message like "fan=3.5" into (device, value).
// Errors: "missing =", "missing device", "bad value".
pub fn parse_reading(text: &str) -> Result<(String, f64), String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_valid_message() {
        assert_eq!(parse_reading("fan=3.5"), Ok(("fan".to_string(), 3.5)));
    }

    #[test]
    fn allows_spaces_around_the_parts() {
        assert_eq!(parse_reading(" oven = 230 "), Ok(("oven".to_string(), 230.0)));
    }

    #[test]
    fn no_equals_sign_is_an_error() {
        assert_eq!(parse_reading("fan 3.5"), Err("missing =".to_string()));
    }

    #[test]
    fn a_missing_device_is_an_error() {
        assert_eq!(parse_reading("=3.5"), Err("missing device".to_string()));
    }

    #[test]
    fn a_value_that_is_not_a_number_is_an_error() {
        assert_eq!(parse_reading("fan=hot"), Err("bad value".to_string()));
    }
}
