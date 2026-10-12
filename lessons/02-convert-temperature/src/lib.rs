// Convert Celsius to Fahrenheit, rounded to one decimal place.
pub fn to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing_point_is_32() {
        assert_eq!(to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn boiling_point_is_212() {
        assert_eq!(to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn minus_40_is_the_same_in_both() {
        assert_eq!(to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn rounds_to_one_decimal() {
        assert_eq!(to_fahrenheit(21.5), 70.7);
        assert_eq!(to_fahrenheit(21.46), 70.6);
    }
}
