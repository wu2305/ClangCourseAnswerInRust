use std::io;
fn main() {
    println!("Calculate the average score");
    println!("Please input the scores with whitespace between");
    let mut scores = String::new();
    io::stdin().read_line(&mut scores).expect("Failed to read line");
    let score: Vec<f64> = scores.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut s = 0.0;
    for i in &score {
        s = s + i;
    };
    let length = score.len();
    let length:f64 = length as f64;
    let average = s/length;
    println!("The average score is {}",average);


}
