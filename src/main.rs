use std::io;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    println!("Please write down the value for N: ");

    // Declare String to store user input for N value
    let mut n_input: String = String::new();

    // Ask user input for N
    io::stdin()
        .read_line(&mut n_input)
        .expect("Failed to read line");

    // Convert the user input for N (String) into an i32 value
    let n: i32 = n_input
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    // Create the augmented matrix as a Vector of Vectors
    let mut augmented_matrix: Vec<Vec<i32>> = Vec::new();

    // Iterate over the matrix and ask user input
    // for coefficients and the constant of each equation
    for i in 0..n {
        println!("Equation number:{}", i + 1);

        //Create an inner vector
        let mut inner_vector: Vec<i32> = Vec::new();

        for j in 0..=n {
            println!("Please enter the coeficient for the variable {}", j + 1);

            // input for the coefficient value and convertion into i32 type
            let mut coefficient_input: String = String::new();
            io::stdin()
                .read_line(&mut coefficient_input)
                .expect("Failed to read line");

            let coefficient: i32 = coefficient_input
                .trim()
                .parse()
                .expect("Failed to parse number: Not a valid integer");

            //assign the coefficient value into the inner vector
            inner_vector.push(coefficient);
        }

        //insert the inner vector (equation) into the matrix
        augmented_matrix.push(inner_vector);
    }

    //show augmented matrix
    for i in 0..n {
        for j in 0..=n {
            print!("[{}],", augmented_matrix[i as usize][j as usize]);
        }
        println!();
    }
}
