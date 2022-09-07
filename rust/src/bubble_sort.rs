pub fn bubble_sort(mut vector: Vec<i32>) -> Vec<i32> {
    let mut swapped = false;
    let mut n = 0;

    while {
        swapped = false;
        n += 1;
        for index in 0..vector.len() {
            if let Some(current) = vector.get(index) {
                if let Some(next_item) = vector.get(index + 1) {
                    n += 1;
                    if current > next_item {
                        vector.swap(index, index + 1);
                        swapped = true;
                    }
                }
            }
        }

        println!("n({}): O({})", vector.len(), n);
        swapped
    } {}

    vector
}

mod tests {
    use super::*;

    #[test]
    fn best_case() {
        let elements = vec![2, 2, 11, 22, 23, 33, 55, 1234, 2342];

        assert_eq!(
            bubble_sort(elements),
            [2, 2, 11, 22, 23, 33, 55, 1234, 2342]
        )
    }
    #[test]
    fn worst_case() {
        let elements = vec![11, 10, 9, 8, 7, 6, 5, 5, 4, 3, 2, 1];

        assert_eq!(
            bubble_sort(elements),
            [1, 2, 3, 4, 5, 5, 6, 7, 8, 9, 10, 11]
        )
    }
}
