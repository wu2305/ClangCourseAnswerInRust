use std::io;
extern crate exitcode;
fn main() {
    println!("Please input the amount of number");
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to read line");
    let mut i:i32 = match i.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please input the number");
            std::process::exit(exitcode::DATAERR);
        },
    };
    if i <= 0 {
        println!("Please input a number bigger than 0");
        std::process::exit(exitcode::DATAERR);
    };
    println!{"Now input the numbers"};
    let mut v:Vec<i32> = Vec::new();
    loop{
        let mut input_num = String::new();
        io::stdin().read_line(&mut input_num).expect("Failed to read line");
        let a:i32 = match input_num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input the number");
                continue;
            },
        };
        v.push(a);
        i = i - 1;
        if i == 0 {
            break;
        }else{
            continue;
        };
    };
    println!("The Vec is {:?}",v);
    let amount = v.len() as f64;
    let mut s = 0.0;
    for num in 0..v.len()  {
        s = s + v[num] as f64;
    };
    let average = s/amount ;
    println!("The average of the Vec is {}.\nThe max number in the Vec is {:?}",average,*v.iter().max_by_key(|x| x.abs()).unwrap());

}
