use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hashmap: HashMap<i32, Vec<usize>> = HashMap::new(); 
    
    for (i, num) in nums.iter().enumerate() {
        let indices = hashmap.entry(*num).or_insert(vec![]);
        indices.push(i);
    }

    println!("{:?}", hashmap);

    let mut result: bool = false;

    // Tagged to allow breaking outer loops as well
    'outer: for num in hashmap {
        let (_, v) = num;
        let v2 = v.clone();
        for (i, previous_index) in v2.iter().enumerate() {
            if i < v.len() - 1 {
                for j in &v[i+1..] {
                    if usize::abs_diff(*j, *previous_index) <= k as usize {
                        result = true;
                        // breaks loops
                        break 'outer;
                    };
                }
            }
        }
    }

    result
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