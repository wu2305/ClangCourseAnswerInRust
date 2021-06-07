fn main() {
    println!("Print out the natrual constant");
    let mut s = 1.0;
    let mut e = 1.0;
    for i in 1..1000 {

        s = s*i as f64;
        e = e + 1.0/s;
    };
    println!("{}",e);
}
