use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn find_duplicates_presorted(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if nums.is_empty() {
        return result;
    }

    nums.sort_unstable();

    for i in 0..(nums.len() - 1) {
        if nums[i] == nums[i + 1] && result.binary_search(&nums[i]).is_err() {
            result.push(nums[i]);
        }
    }

    result
}

#[allow(dead_code)]
fn find_duplicates_hashmap(nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result: HashSet<i32> = HashSet::new();

    if nums.is_empty() {
        return Vec::new();
    }

    for i in nums {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > 1 {
            result.insert(i);
        }
    }

    result.into_iter().collect()
}

#[allow(dead_code)]
fn find_duplicates_allocated(nums: Vec<i32>) -> Vec<i32> {
    let mut arr = vec![0; nums.len() - 1];
    let mut result: HashSet<i32> = HashSet::new();
    if nums.is_empty() {
        return Vec::new();
    }

    for i in nums {
        if i <= 0 {
            panic!("Only signed values is supported")
        }

        let a = &mut arr[(i - 1) as usize];
        *a += 1;
        if *a > 1 {
            result.insert(i);
        }
    }

    result.into_iter().collect()
}

mod tests {
    use super::*;

    #[test]
    fn empty_array_duplicates() {
        assert_eq!(find_duplicates_presorted(vec![]), vec![])
    }

    #[test]
    fn hashmap_duplicates() {
        assert_eq!(find_duplicates_hashmap(vec![1, 2, 2, 3, 4, 5, 2]), vec![2])
    }

    #[test]
    fn allocated_arr_deduplication() {
        assert_eq!(
            find_duplicates_allocated(vec![1, 2, 2, 3, 4, 5, 2]),
            vec![2]
        )
    }
}
