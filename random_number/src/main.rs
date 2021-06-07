use rand::Rng;
fn main() {
    let mut i = 0;
    loop{
        let a = rand::thread_rng().gen_range(0,100);
        println!("{}",a);
        i = i+1;
        if i == 10 {
            break;
        };
    };
}
