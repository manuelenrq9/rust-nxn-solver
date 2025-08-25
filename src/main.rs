use std::io;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    println!("Please write down the value for N: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    let augmented_matrix: Vec<Vec<i32>> = Vec::new();
    for i in 0..n {
        println!("Equation number:{}", i + 1);
        for j in 0..=n {
            println!("Please enter the coeficient for the variable {}", j + 1);
        }
    }
}
