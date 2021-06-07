use std::io;
fn main() {
    println!("sort the string");
    println!("Please input the string");
    let mut wordy = String::new();
    io::stdin().read_line(&mut wordy).expect("Failed to read line");
    let wordy = wordy.trim();
    let mut chars:Vec<char> = wordy.chars().collect();
    chars.sort();
    println!("{:?}",chars);

}
