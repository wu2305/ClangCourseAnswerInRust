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
        println!("Then input the number want to insert");
        let mut insert_num = String::new();
        io::stdin().read_line(&mut insert_num).expect("Failed to read line");
        let b:i32 = match insert_num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            },
        };
        v.push(b);
        break;
    };
    v.sort();
    println!("The new vec is {:?}",v);

}
