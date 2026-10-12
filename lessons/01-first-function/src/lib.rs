// Your first Rust function. Replace todo!(), then run the checks.

/// Return a greeting like "Hello, Sam!".
pub fn greeting(name: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn says_hello_to_sam() {
        assert_eq!(greeting("Sam"), "Hello, Sam!");
    }

    #[test]
    fn works_for_any_name() {
        assert_eq!(greeting("Ada Lovelace"), "Hello, Ada Lovelace!");
    }

    #[test]
    fn returns_a_string_you_own() {
        let mut text = greeting("Sam");
        text.push('?'); // only possible because the String is yours
        assert_eq!(text, "Hello, Sam!?");
    }
}
