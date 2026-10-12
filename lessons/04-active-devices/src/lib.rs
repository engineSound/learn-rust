// Pick the names of the online devices, sorted alphabetically.

pub struct Device {
    pub name: String,
    pub online: bool,
}

pub fn active_names(devices: &[Device]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn devices() -> Vec<Device> {
        vec![
            Device { name: "porch light".into(), online: true },
            Device { name: "garage door".into(), online: false },
            Device { name: "attic fan".into(), online: true },
        ]
    }

    #[test]
    fn keeps_only_online_devices() {
        assert_eq!(active_names(&devices()).len(), 2);
    }

    #[test]
    fn sorts_the_names_alphabetically() {
        assert_eq!(active_names(&devices()), vec!["attic fan", "porch light"]);
    }

    #[test]
    fn the_caller_keeps_its_devices() {
        let list = devices();
        active_names(&list);
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].name, "porch light");
    }

    #[test]
    fn no_devices_means_no_names() {
        assert_eq!(active_names(&[]), Vec::<String>::new());
    }
}
