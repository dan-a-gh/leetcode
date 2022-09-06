// Leetcode problem 242 submission

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = t.chars().collect();

    s.sort_unstable();
    t.sort_unstable();

    if s == t {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn positive_case() {
        let string1: String = String::from("Rust is cool");
        let string2: String = String::from("looc si tsuR");

        assert!(is_anagram(string1, string2));
    }

    #[test]
    fn negative_case() {
        let string1: String = String::from("these are not alike");
        let string2: String = String::from("these are not the same");

        assert!(!(is_anagram(string1, string2)));
    }
}
