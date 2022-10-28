pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    nums.sort_unstable();

    for (smallest_i, smallest) in nums.iter().enumerate() {
        // This skips duplicate values in sorted vec
        if smallest_i > 0 && *smallest == nums[smallest_i - 1] {
            continue;
        }

        let smallest: i32 = *smallest;

        // Starts in the next position in nums after smallest
        let mut left_smaller_i: usize = smallest_i + 1;
        // Starts in last position of nums
        let mut right_bigger_i: usize = nums.len() - 1;

        while left_smaller_i < right_bigger_i {
            let three_sum: i32 = smallest + nums[left_smaller_i] + nums[right_bigger_i];
            if three_sum > 0 {
                right_bigger_i -= 1;
            } else if three_sum < 0 {
                left_smaller_i += 1;
            } else {
                result.push(vec![smallest, nums[left_smaller_i], nums[right_bigger_i]]);
                left_smaller_i += 1;
                while nums[left_smaller_i] == nums[left_smaller_i - 1] && left_smaller_i < right_bigger_i {
                    left_smaller_i += 1;
                }
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn example_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(three_sum(nums), output);
    }

    #[test]
    fn example_2() {
        let nums = vec![0, 1, 1];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(nums), output);
    }

    #[test]
    fn example_3() {
        let nums = vec![0, 0, 0];
        let output = vec![vec![0, 0, 0]];
        assert_eq!(three_sum(nums), output)
    }
}