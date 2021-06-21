use std::io;
fn main(){
    println! ("Please input the 2D vec");
    println! ("The first line");
    let mut input_numbers = String::new();
    io::stdin().read_line(&mut input_numbers).expect("Failed to read line");
    let arr:Vec<i32> = input_numbers.split_whitespace().map(|input_numbers| input_numbers.parse().unwrap()).collect();
    let mut matrix:Vec<_> = Vec::new();

}