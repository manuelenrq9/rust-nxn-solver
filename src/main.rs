use std::io;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    println!("Please write down the value for N: ");
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input);
    println!("N is {}", n_input);
}
