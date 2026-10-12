// Count how many readings each device sent. Replace todo!(), then run the checks.
use std::collections::HashMap;

/// [("hall", 21.5), ("kitchen", 19.0), ("hall", 22.0)] → {"hall": 2, "kitchen": 1}
pub fn count_by_device(readings: &[(String, f64)]) -> HashMap<String, usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(device: &str, value: f64) -> (String, f64) {
        (device.to_string(), value)
    }

    #[test]
    fn counts_each_device() {
        let counts = count_by_device(&[r("hall", 21.5), r("kitchen", 19.0), r("hall", 22.0)]);
        assert_eq!(counts.get("hall"), Some(&2));
        assert_eq!(counts.get("kitchen"), Some(&1));
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn no_readings_means_an_empty_map() {
        assert!(count_by_device(&[]).is_empty());
    }

    #[test]
    fn a_device_that_never_reported_is_missing() {
        let counts = count_by_device(&[r("hall", 21.5)]);
        assert_eq!(counts.get("garage"), None);
    }

    #[test]
    fn the_readings_are_still_yours() {
        let readings = vec![r("hall", 21.5)];
        count_by_device(&readings);
        assert_eq!(readings.len(), 1); // only borrowed
    }
}
