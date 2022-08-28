mod p217;

fn main() {
    let nums: Vec<i32> = vec![2, -3, 13, 6];
    let result: bool = p217::contains_duplicate(nums);
    println!("Contains duplicate?: {}", result);
}
