// Lesson 01: bindings and expressions.
// Replace each todo!() with your implementation, then run: cargo test

/// Convert Celsius to Fahrenheit: F = C × 9/5 + 32.
/// Write the body as a single expression: no `return`, no trailing `;`.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

/// Describe a number as "negative", "zero" or "positive".
/// Use `if` as an expression.
pub fn classify(n: i32) -> &'static str {
    todo!()
}

/// Sum of the even numbers from 0 to n inclusive, using a `let mut` total
/// and a `for` loop over the range `0..=n`.
/// The result is u64 so large n doesn't overflow: you'll need a conversion.
pub fn sum_of_evens(n: u32) -> u64 {
    todo!()
}

/// Parse a string like " 21 " and return the number doubled.
/// Use shadowing: trim, then parse into an i64, reusing the name `s`.
/// You may use `.unwrap()` for now; lesson 05 replaces it with real error handling.
pub fn parse_and_double(s: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing_and_boiling() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn minus_forty_is_the_same_in_both() {
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn classify_all_three() {
        assert_eq!(classify(-7), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(3), "positive");
    }

    #[test]
    fn sum_of_evens_small() {
        assert_eq!(sum_of_evens(0), 0);
        assert_eq!(sum_of_evens(10), 30); // 0+2+4+6+8+10
    }

    #[test]
    fn sum_of_evens_large_does_not_overflow_u32() {
        assert_eq!(sum_of_evens(200_000), 10_000_100_000);
    }

    #[test]
    fn parse_and_double_trims_spaces() {
        assert_eq!(parse_and_double(" 21 "), 42);
        assert_eq!(parse_and_double("-5"), -10);
    }
}
