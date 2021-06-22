use std::io;
fn main(){
    println! ("Please input the 2D vec");
    println! ("The first line");
    let mut s = String::new();
    println!("Input the matrix line by line");
    io::stdin().read_line(&mut s).expect("Failed to read the line");
    let arr:Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut matrix : Vec<Vec<i32>> = vec![vec![0i32;arr.len()];arr.len()];
    let length = arr.len();
    for i in 0..length{
        matrix[0][i] = arr[i];
    };
    for i in 1..length{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let arr:Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        for j in 0..length{
            matrix[i][j] = arr[j];
        };
    };
    println!("The inputted matrix is {:?}",matrix);
    let mut trans_matrix:Vec<Vec<i32>> = vec![vec![0i32;length];length];
    for i in 0..length{
        for j in 0..length{
            trans_matrix[j][i] = matrix[i][j];
        };
    };
    if trans_matrix == matrix {
        println!("This is a symmetry matrix");
    }else{
        println!("This is not a symetry matrix");
    };
}