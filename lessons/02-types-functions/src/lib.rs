// Lesson 02: types, tuples and slices. Replace each todo!(), then run the tests.

/// Convert to a byte, clamping to 0..=255 instead of wrapping.
/// to_byte(-5) == 0, to_byte(300) == 255, to_byte(65) == 65.
pub fn to_byte(x: i32) -> u8 {
    todo!()
}

/// Smallest and largest of three numbers, as a tuple (min, max).
pub fn min_max3(a: i32, b: i32, c: i32) -> (i32, i32) {
    todo!()
}

/// Split a number of seconds into (hours, minutes, seconds).
/// split_seconds(3725) == (1, 2, 5).
pub fn split_seconds(total: u64) -> (u64, u64, u64) {
    todo!()
}

/// The arithmetic mean of a slice; 0.0 for an empty slice.
pub fn mean(values: &[f64]) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_byte_clamps_instead_of_wrapping() {
        assert_eq!(to_byte(65), 65);
        assert_eq!(to_byte(-5), 0);
        assert_eq!(to_byte(300), 255);
        assert_eq!(to_byte(i32::MAX), 255);
    }

    #[test]
    fn min_max3_any_order() {
        assert_eq!(min_max3(3, 1, 2), (1, 3));
        assert_eq!(min_max3(-1, -1, -1), (-1, -1));
        assert_eq!(min_max3(5, 9, 7), (5, 9));
    }

    #[test]
    fn split_seconds_into_parts() {
        assert_eq!(split_seconds(3725), (1, 2, 5));
        assert_eq!(split_seconds(59), (0, 0, 59));
        assert_eq!(split_seconds(90_000), (25, 0, 0));
    }

    #[test]
    fn mean_of_values_and_of_nothing() {
        assert_eq!(mean(&[1.0, 2.0, 4.5]), 2.5);
        assert_eq!(mean(&[]), 0.0);
        let v = vec![10.0, 20.0];
        assert_eq!(mean(&v), 15.0);
    }
}
