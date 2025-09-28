use std::io;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    println!("Please write down the value for N: ");
    let n: i32 = get_i32_from_user();
    let mut matrix: Vec<Vec<f64>> = create_augmented_matrix(&n);
    show_augmented_matrix(&matrix, &n);
    println!("===============================================================");
    println!("===============================================================");
    solve_augmented_matrix(&mut matrix, n);
    show_augmented_matrix(&matrix, &n);
}

fn create_augmented_matrix(n: &i32) -> Vec<Vec<f64>> {
    // Create the augmented matrix as a Vector of Vectors
    let mut augmented_matrix: Vec<Vec<f64>> = Vec::new();

    // Iterate over the matrix and ask user input
    // for coefficients and the constant of each equation
    for i in 0..*n {
        println!("Equation number:{}", i + 1);

        //Create an inner vector
        let mut inner_vector: Vec<f64> = Vec::new();

        for j in 0..=*n {
            println!("Please enter the coeficient for the variable {}", j + 1);
            let coefficient: f64 = get_f64_from_user();

            //assign the coefficient value into the inner vector
            inner_vector.push(coefficient);
        }
        //insert the inner vector (equation) into the matrix
        augmented_matrix.push(inner_vector);
    }
    return augmented_matrix;
}

fn get_i32_from_user() -> i32 {
    let mut s: String = String::new();

    //get user input as a String and store it in s
    io::stdin().read_line(&mut s).expect("Failed to read line");

    // Convert the user input (String) into an f64 value
    let n: i32 = convert_string_to_i32(s);

    return n;
}

fn get_f64_from_user() -> f64 {
    let mut s: String = String::new();

    //get user input as a String and store it in s
    io::stdin().read_line(&mut s).expect("Failed to read line");

    // Convert the user input (String) into an f64 value
    let n: f64 = convert_string_to_f64(s);

    return n;
}

fn convert_string_to_i32(s: String) -> i32 {
    let n: i32 = s
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    return n;
}

fn convert_string_to_f64(s: String) -> f64 {
    let n: f64 = s
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    return n;
}

fn show_augmented_matrix(matrix: &Vec<Vec<f64>>, n: &i32) {
    for i in 0..*n {
        for j in 0..=*n {
            print!("[{}],", matrix[i as usize][j as usize]);
        }
        println!();
    }
}

fn solve_augmented_matrix(matrix: &mut Vec<Vec<f64>>, n: i32) {
    //first we solve the lower triangle by doing forward elimination
    //set the pivot column for each column on the left side of the matrix
    for k in 0..n {
        //check if the value at [j][j](main diagonal) is different from 0
        // if it its, set it as the pivot element
        let pivot_value: f64 = matrix[k as usize][k as usize];
        if pivot_value != 0.0 {
            // Apply a division through the whole row, to make the pivot element equal to 1
            for j in 0..=n {
                matrix[k as usize][j as usize] /= pivot_value;
            }
            show_augmented_matrix(matrix, &n);
            println!("===============================================================");
            println!("===============================================================");
            //make all values below the pivot element equal to zero,
            //by substracting the pivot element's row multiplied by
            //the value you wanto to turn to zero
            for i in k + 1..n {
                let row_multiplier: f64 = matrix[i as usize][k as usize];
                for j in 0..=n {
                    let substracting_value: f64 = matrix[k as usize][j as usize] * row_multiplier;
                    matrix[i as usize][j as usize] -= substracting_value;
                    show_augmented_matrix(matrix, &n);
                    println!("===============================================================");
                    println!("===============================================================");
                }
                show_augmented_matrix(matrix, &n);
                println!("===============================================================");
                println!("===============================================================");
            }
            show_augmented_matrix(matrix, &n);
            println!("===============================================================");
            println!("===============================================================");
        }
    }
    //backward elimination for solving the upper triangle
    for k in (0..n).rev() {
        let pivot_value: f64 = matrix[k as usize][k as usize];
        if pivot_value != 0.0 {
            //make all values above the pivot element equal to zero,
            //by substracting the pivot element's row multiplied by
            //the value you wanto to turn to zero
            for i in (0..k).rev() {
                let row_multiplier: f64 = matrix[i as usize][k as usize];
                for j in 0..=n {
                    let substracting_value: f64 = matrix[k as usize][j as usize] * row_multiplier;
                    matrix[i as usize][j as usize] -= substracting_value;
                }
            }
        }
    }
}
