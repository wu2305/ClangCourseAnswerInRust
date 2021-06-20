fn main() {
    println!("combine twwo sentence");
    let mut sentence1:String = "AABBCC".to_owned();
    let sentence2:&str = "DDEEFF";
    sentence1.push_str(sentence2);
    println!("{}",sentence1);
}
