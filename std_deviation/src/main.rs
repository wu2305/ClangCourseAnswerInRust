use std::io;
fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum/count as f32),
        _ => None,
    }
}
fn std_deviation(data: &[i32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>()/count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}
fn main() {
    println!("Calculate the standard deviation");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let data:Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let data_mean = mean(&data);
    println!("Mean is {:?}", data_mean);
    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {:?}", data_std_deviation);
}
