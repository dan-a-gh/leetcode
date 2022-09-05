mod p217;
mod p242;
mod p1;
mod p49;

fn main() {
    // p217
    let nums: Vec<i32> = vec![2, -3, 13, 6];
    let answer: bool = p217::contains_duplicate(nums);
    println!("p217: Contains duplicate?: {}", answer);

    // p242
    let string1: String = String::from("Hello, world");
    let string2: String = String::from("dlrow, olleH");

    let answer: bool = p242::is_anagram(string1, string2);
    println!("p242: Is anagram?: {}", answer);

    // p49
    let strs = vec!["eat".to_string(), "tea".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    let answer = p49::group_anagrams(strs);
    println!("p49: Grouped anagrams:{:?}", answer);
}
