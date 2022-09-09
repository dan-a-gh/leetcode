// p125 Valid Palindrome
pub fn is_palindrome(s: String) -> bool {
    let iter = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    iter.clone().eq(iter.rev())
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