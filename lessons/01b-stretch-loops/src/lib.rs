// Stretch: loops that return values. Replace each todo!(), then run the tests.

/// Collatz steps: how many steps it takes n to reach 1, where each step is
/// n / 2 if n is even, otherwise 3n + 1. collatz_steps(1) == 0, collatz_steps(6) == 8.
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

/// Sum of the decimal digits: digit_sum(1234) == 10, digit_sum(0) == 0.
pub fn digit_sum(n: u64) -> u64 {
    todo!()
}

/// The smallest power of two that is >= n (n >= 1), found with
/// `let p = loop { ... break ... };`. next_power_of_two(5) == 8, next_power_of_two(8) == 8.
/// (Don't call the built-in `u64::next_power_of_two`.)
pub fn next_power_of_two(n: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_small_numbers() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
    }

    #[test]
    fn collatz_needs_wide_integers() {
        // Climbs past 2^32 on the way: a u32 would overflow and panic.
        assert_eq!(collatz_steps(837_799), 524);
    }

    #[test]
    fn digit_sums() {
        assert_eq!(digit_sum(0), 0);
        assert_eq!(digit_sum(1234), 10);
        assert_eq!(digit_sum(u64::MAX), 87);
    }

    #[test]
    fn powers_of_two() {
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(8), 8);
        assert_eq!(next_power_of_two(1_000_000), 1_048_576);
    }
}
