use std::collections::HashMap;

pub fn two_numbers_hashmap(array: &[isize], sum: isize) -> (isize, isize) {
    let mut acc = HashMap::new();

    for num in array.iter() {
        let summand = sum - num;

        match acc.get(&summand) {
            None => acc.insert(summand, true),
            Some(_value) => return (*num, summand),
        };
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_right_two_numbers_found() {
        assert_eq!(
            two_numbers_hashmap(&[3, 5, -4, 8, 11, 11, -1, 6], 10),
            (11, -1)
        )
    }
}
