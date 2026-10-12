// Who owns the reading? Replace each todo!(), then run the checks.

/// Borrow the reading and return a new String with "°C" added.
/// with_unit("21.5") == "21.5°C"
pub fn with_unit(reading: &str) -> String {
    todo!()
}

/// Borrow the readings and add them up; the caller keeps its Vec.
/// total(&[1.0, 2.5, 3.0]) == 6.5
pub fn total(readings: &[f64]) -> f64 {
    todo!()
}

/// Append the value to the caller's log, keeping only the last 3 readings.
/// [1.0, 2.0, 3.0] + 4.0 → [2.0, 3.0, 4.0]
pub fn record(log: &mut Vec<f64>, value: f64) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_unit_adds_degrees() {
        assert_eq!(with_unit("21.5"), "21.5°C");
    }

    #[test]
    fn the_caller_keeps_its_text() {
        let reading = String::from("19");
        let shown = with_unit(&reading);
        assert_eq!(reading, "19"); // still usable: it was only borrowed
        assert_eq!(shown, "19°C");
    }

    #[test]
    fn total_adds_and_the_vec_is_still_usable() {
        let readings = vec![1.0, 2.5, 3.0];
        assert_eq!(total(&readings), 6.5);
        assert_eq!(readings.len(), 3); // still ours
    }

    #[test]
    fn total_of_nothing_is_zero() {
        assert_eq!(total(&[]), 0.0);
    }

    #[test]
    fn record_appends_to_the_callers_log() {
        let mut log = vec![1.0];
        record(&mut log, 2.0);
        assert_eq!(log, vec![1.0, 2.0]);
    }

    #[test]
    fn record_keeps_only_the_last_three() {
        let mut log = vec![1.0, 2.0, 3.0];
        record(&mut log, 4.0);
        assert_eq!(log, vec![2.0, 3.0, 4.0]);
    }
}
