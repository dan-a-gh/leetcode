pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let ref_nums = &nums;

    for (i, val_i) in ref_nums.iter().enumerate() {
        let mut rm_i_nums = nums.clone();
        rm_i_nums.remove(i);
        
        for (j, val_j) in rm_i_nums.iter().enumerate() {
            let mut rm_j_nums = rm_i_nums.clone();
            rm_j_nums.remove(j);
            
            for k in rm_j_nums {
                if val_i + val_j + k != 0 {
                    continue;
                } else {
                    let mut output = vec![*val_i, *val_j, k];
                    output.sort_unstable();
                    result.push(output);
                }
            }
        }
    }

    result.sort_unstable();
    result.dedup();

    result
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