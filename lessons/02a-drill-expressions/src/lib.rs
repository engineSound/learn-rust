// Drill: expressions and shadowing. Replace each todo!(), then run the tests.

/// Letter grade: 90 and up 'A', 80 and up 'B', 70 and up 'C', otherwise 'F'.
/// Write it as one `if` / `else if` expression.
pub fn grade(score: u32) -> char {
    todo!()
}

/// The distance between two numbers, always non-negative.
/// Use `if` as an expression (don't call `abs` or `abs_diff`).
pub fn distance(a: i32, b: i32) -> i32 {
    todo!()
}

/// Sum a comma-separated list like "3, 4 ,5" → 12.
/// Loop over `s.split(',')`, shadow each piece with its trimmed version, then
/// parse it into an i64 (`.unwrap()` is fine for now).
pub fn parse_sum(s: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_boundaries() {
        assert_eq!(grade(100), 'A');
        assert_eq!(grade(90), 'A');
        assert_eq!(grade(89), 'B');
        assert_eq!(grade(70), 'C');
        assert_eq!(grade(69), 'F');
        assert_eq!(grade(0), 'F');
    }

    #[test]
    fn distance_either_way_round() {
        assert_eq!(distance(3, 10), 7);
        assert_eq!(distance(10, 3), 7);
        assert_eq!(distance(-5, 5), 10);
        assert_eq!(distance(4, 4), 0);
    }

    #[test]
    fn parse_sum_trims_each_piece() {
        assert_eq!(parse_sum("3, 4 ,5"), 12);
        assert_eq!(parse_sum("-2,2"), 0);
        assert_eq!(parse_sum(" 42 "), 42);
    }
}
