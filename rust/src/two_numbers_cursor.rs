pub fn two_numbers_cursors(array: &[isize], sum: isize) -> (isize, isize) {
    return (0, 0);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_right_two_numbers_found() {
        assert_eq!(
            two_numbers_cursors(&[3, 5, -4, 8, 11, 11, -1, 6], 10),
            (11, -1)
        )
    }
}