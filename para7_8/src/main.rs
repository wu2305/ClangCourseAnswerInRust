use std::io;
extern crate exitcode;
fn main() {
    println!("Calculate the factorial");
    println!("Please input the number");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed tto read the line");
    let number:i64 = number.trim().parse().unwrap();
    if number <= 0 {
        println!("Please input an number bigger than 0");
        std::process::exit(exitcode::DATAERR);
    };
    let mut s = 1;
    for i in 1..number+1 {
        s = s * i;
    };
    println!("{}",s);
}
