// Summarize a day of readings. Replace todo!(), then run the checks.
use std::collections::HashMap;

pub struct Reading {
    pub device: String,
    pub celsius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Summary {
    pub total: usize,
    pub by_device: HashMap<String, usize>,
    pub hottest: Option<(String, f64)>,
}

pub fn summarize(readings: &[Reading]) -> Summary {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(device: &str, celsius: f64) -> Reading {
        Reading { device: device.to_string(), celsius }
    }

    #[test]
    fn counts_all_readings() {
        let s = summarize(&[r("hall", 21.5), r("kitchen", 25.0), r("hall", 22.0)]);
        assert_eq!(s.total, 3);
    }

    #[test]
    fn counts_per_device() {
        let s = summarize(&[r("hall", 21.5), r("kitchen", 25.0), r("hall", 22.0)]);
        assert_eq!(s.by_device.get("hall"), Some(&2));
        assert_eq!(s.by_device.get("kitchen"), Some(&1));
    }

    #[test]
    fn finds_the_hottest_reading() {
        let s = summarize(&[r("hall", 21.5), r("kitchen", 25.0), r("hall", 22.0)]);
        assert_eq!(s.hottest, Some((String::from("kitchen"), 25.0)));
    }

    #[test]
    fn a_tie_keeps_the_first() {
        let s = summarize(&[r("hall", 25.0), r("kitchen", 25.0)]);
        assert_eq!(s.hottest, Some((String::from("hall"), 25.0)));
    }

    #[test]
    fn handles_freezing_days() {
        let s = summarize(&[r("garden", -4.0), r("shed", -2.5)]);
        assert_eq!(s.hottest, Some((String::from("shed"), -2.5)));
    }

    #[test]
    fn an_empty_day_has_no_hottest() {
        let s = summarize(&[]);
        assert_eq!(s, Summary { total: 0, by_device: HashMap::new(), hottest: None });
    }
}
