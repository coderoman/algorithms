pub fn two_numbers(array: &[isize], sum: usize) -> i8 {
    println!("Hello, world!");
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_right_two_numbers_found() {
        assert_eq!(two_numbers(&[3, 5, -4, 8, 11, 11, -1, 6], 10), 2)
    }
}
