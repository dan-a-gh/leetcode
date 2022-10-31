use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hashmap = HashMap::new(); 
    for (i, num) in nums.iter().enumerate() {
        if let Some(j) = hashmap.insert(num, i) {
            if (i as i32) - (j as i32) <= k {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::contains_nearby_duplicate;

    #[test]
    fn example_1() {
        let nums = vec![1,2,3,1];
        let k = 3;
        assert!(contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn example_2() {
        let nums = vec![1,0,1,1];
        let k = 1;
        assert!(contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn example_3() {
        let nums = vec![1,2,3,1,2,3];
        let k = 2;
        assert!(!(contains_nearby_duplicate(nums, k)));
    }
}