use std::io;
fn main(){
    println!("Please input the String");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s:Vec<u8> = s.trim().bytes().collect();
    let mut nums:Vec<u8> = Vec::new();
    for i in 0..s.len() {
        if s[i]<=9 {
            nums.push(s[i]);
        };
    };
    let nums = String::from_utf8(nums).expect("Failed to transfer the String");
    println!("{}",nums);
}