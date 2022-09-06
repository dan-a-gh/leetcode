use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&j) = hm.get(&diff) {
            return vec![j as i32, i as i32];
        } else {
            hm.insert(*num, i as i32);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(two_sum(nums, target), [0, 1]);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(two_sum(nums, target), [1, 2])
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(two_sum(nums, target), [0, 1])
    }
}
