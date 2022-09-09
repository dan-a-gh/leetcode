// p125 Valid Palindrome
pub fn is_palindrome(s: String) -> bool {
    let s1: String = s.to_lowercase();
    let s1: String = s1.chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .collect::<String>();
    
    let s2 = s1.clone()
        .chars()
        .rev()
        .collect::<String>();

    s1 == s2
}

#[cfg(test)]
mod test {
    use super::is_palindrome;

    #[test]
    fn example_1() {
        let s: String = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome(s));
    }

    #[test]
    fn example_2() {
        let s: String = String::from("race a car");
        assert!(!(is_palindrome(s)));
    }

    #[test]
    fn example_3() {
        let s: String = String::from(" ");
        assert!(is_palindrome(s));
    }

    #[test]
    fn custom_example_1() {
        let s: String = String::from("no lemon, no melon");
        assert!(is_palindrome(s));
    }
}