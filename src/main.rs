use std::io;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    let matrix: Vec<Vec<i32>> = create_augmented_matrix();
}

fn create_augmented_matrix() -> Vec<Vec<i32>> {
    // Request and get user input for N
    println!("Please write down the value for N: ");
    let n: i32 = get_i32_from_user();

    // Create the augmented matrix as a Vector of Vectors
    let mut augmented_matrix: Vec<Vec<i32>> = Vec::new();

    // Iterate over the matrix and ask user input
    // for coefficients and the constant of each equation
    for i in 0..n {
        println!("Equation number:{}", i + 1);

        //Create an inner vector
        let mut inner_vector: Vec<i32> = Vec::new();

        for j in 0..=n {
            // input for the coefficient value
            println!("Please enter the coeficient for the variable {}", j + 1);
            let coefficient: i32 = get_i32_from_user();

            //assign the coefficient value into the inner vector
            inner_vector.push(coefficient);
        }
        //insert the inner vector (equation) into the matrix
        augmented_matrix.push(inner_vector);
    }

    show_augmented_matrix(&augmented_matrix, n);
    return augmented_matrix;
}

fn get_i32_from_user() -> i32 {
    //Declare String value
    let mut s: String = String::new();

    //get user input as a String and store it in s
    io::stdin().read_line(&mut s).expect("Failed to read line");

    // Convert the user input (String) into an i32 value
    let n: i32 = convert_string_to_i32(s);

    return n;
}

fn convert_string_to_i32(s: String) -> i32 {
    let n: i32 = s
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    return n;
}

fn show_augmented_matrix(matrix: &Vec<Vec<i32>>, n: i32) {
    for i in 0..n {
        for j in 0..=n {
            print!("[{}],", matrix[i as usize][j as usize]);
        }
        println!();
    }
}
