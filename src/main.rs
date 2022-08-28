mod p217;
mod p242;

fn main() {
    // p217
    let nums: Vec<i32> = vec![2, -3, 13, 6];
    let answer: bool = p217::contains_duplicate(nums);
    println!("Contains duplicate?: {}", answer);

    // p242
    let string1: String = String::from("Hello, world");
    let string2: String = String::from("dlrow, olleH");

    let answer: bool = p242::is_anagram(string1, string2);
    println!("Is anagram?: {}", answer);
}
