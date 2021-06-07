use std::io;
use rand::Rng;
extern crate exitcode;
fn main() {
    println!("How many random number you want to create?");
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to read line");
    let i:i32 = match i.trim().parse(){
        Ok(num) => num;
        Err(_) => {
            println!("Please input the number");
            std::process::exit(exitcode::DATAERR);
        },
    };
    if i <= 0 {
        println!("Please input the number bigger than 0");
        std::process::exit(exitcode::DATAERR);
    };
    let mut v:Vec<i32> = Vec::new();
    loop {
        let a = rand::thread_rng().gen_range(1,101);
        v.push(a);
        i = i - 1;
        if i == 0 {
            break;
        }else{
            continue;
        };
    };
    for num in 0..v.len() {
        if 50 <= v[num] < 60 || v[num]%5 == 0 {
            let v5:Vec<i32> = v[num].collect();
        }else{
            continue;
        };
        println!("The numbers contain 5 is {:?}",v5);
    };

}
