// Describe a device status in words. Replace todo!(), then run the checks.

pub enum Status {
    Online,
    Offline { since_minutes: u32 },
    Error(String),
}

/// "online", "offline for 5 min" or "error: overheated"
pub fn describe(status: &Status) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_says_online() {
        assert_eq!(describe(&Status::Online), "online");
    }

    #[test]
    fn offline_says_for_how_long() {
        assert_eq!(describe(&Status::Offline { since_minutes: 5 }), "offline for 5 min");
    }

    #[test]
    fn error_shows_its_message() {
        assert_eq!(describe(&Status::Error(String::from("overheated"))), "error: overheated");
    }

    #[test]
    fn the_status_is_still_usable_afterwards() {
        let status = Status::Error(String::from("low battery"));
        describe(&status); // only borrowed
        assert!(matches!(status, Status::Error(_)));
    }
}
