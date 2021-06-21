use std::io;
fn main() {
    println!("The fake insert program");
    let mut v:Vec<i32> = Vec::new();
    println!("Please input the original ten scores");
    let mut i = 0;
    loop{
        let mut input_num = String::new();
        io::stdin().read_line(&mut input_num).expect("Failed to read line");
        let a:i32 = match input_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };
        v.push(a);
        i = i + 1;
        if i == 10 {
            break;
        }else{
            continue;
        };
    };
    println!("The Vec is {:?}",v);
    loop{
        println!("Then input the number want to remove");
        let mut remove_num = String::new();
        io::stdin().read_line(&mut remove_num).expect("Failed to read line");
        let remove_num:i32 = match remove_num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };
        v.retain(|&x| x != remove_num);
        break;
    };
    println!("The new vec is {:?}",v);

}
