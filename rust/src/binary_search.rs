use std::cmp::Ordering;

pub fn binary_search(sorted_array: &[i32], value: i32) -> Option<usize> {
    if sorted_array.is_empty() {
        return None;
    }

    let mut left_index = 0;
    let mut right_index = sorted_array.len();

    while left_index < right_index {
        let mid = left_index + (right_index - left_index) / 2;

        match sorted_array[mid].cmp(&value) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => {
                left_index = mid + 1;
            }
            Ordering::Greater => {
                right_index = mid;
            }
        }
    }

    None
}

mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(binary_search(&[1, 2, 3, 4, 5, 6], 3), Some(2))
    }
}
