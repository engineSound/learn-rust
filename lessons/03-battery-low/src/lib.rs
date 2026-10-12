// A battery is low when its level is known, under 20, and it's not charging.
pub fn is_battery_low(level: Option<u8>, charging: bool) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifteen_percent_not_charging_is_low() {
        assert_eq!(is_battery_low(Some(15), false), true);
    }

    #[test]
    fn fifteen_percent_while_charging_is_not_low() {
        assert_eq!(is_battery_low(Some(15), true), false);
    }

    #[test]
    fn exactly_twenty_percent_is_not_low() {
        assert_eq!(is_battery_low(Some(20), false), false);
    }

    #[test]
    fn zero_percent_is_low() {
        assert_eq!(is_battery_low(Some(0), false), true);
    }

    #[test]
    fn an_unknown_level_is_not_low() {
        assert_eq!(is_battery_low(None, false), false);
    }
}
