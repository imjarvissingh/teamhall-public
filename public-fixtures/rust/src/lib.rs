/// A dependency-free fixture used only to confirm that a Rust test can run.
pub fn checked_sum(left: i32, right: i32) -> Option<i32> {
    left.checked_add(right)
}

#[cfg(test)]
mod tests {
    use super::checked_sum;

    #[test]
    fn synthetic_checked_sum_handles_regular_and_overflowing_values() {
        assert_eq!(checked_sum(20, 22), Some(42));
        assert_eq!(checked_sum(i32::MAX, 1), None);
    }
}
