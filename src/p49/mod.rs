// Group Anagrams
// https://leetcode.com/problems/group-anagrams/

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();
    for v in strs {
        let mut k = v.chars().collect::<Vec<char>>();
        k.sort_unstable();
        map.entry(k).or_insert_with(|| vec![]).push(v);
    }
    let mut result = map
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<Vec<String>>>();

    for v in &mut result {
        v.sort_unstable();
    }

    result.sort_unstable();
    result
}

#[cfg(test)]
mod tests {
    use super::group_anagrams;

    #[test]
    fn example_1() {
        let input: Vec<String> = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let result: Vec<Vec<String>> = group_anagrams(input);
        assert_eq!(
            result,
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]
        );
    }

    #[test]
    fn example_2() {
        let input: Vec<String> = vec![String::from("")];
        let result: Vec<Vec<String>> = group_anagrams(input);
        assert_eq!(result, vec![vec![""]]);
    }

    #[test]
    fn example_3() {
        let input: Vec<String> = vec![String::from("a")];
        let result: Vec<Vec<String>> = group_anagrams(input);
        assert_eq!(result, vec![vec!["a"]]);
    }
}
