// Leetcode problem 217 submission

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut nums = nums;
    nums.sort();
    nums.dedup();
    let deduped_len = nums.len();
    if len != deduped_len {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;

    #[test]
    fn positive_case () {
        let x = vec![2, 2, 3, 5];
        assert!(contains_duplicate(x));
    }

    #[test]
    fn negative_case() {
        let y = vec![1, 2, 3, 4];
        assert!(!(contains_duplicate(y)))
    }
}