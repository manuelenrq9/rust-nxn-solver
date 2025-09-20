use std::io;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    println!("Please write down the value for N: ");
    let n: i32 = get_i32_from_user();
    let mut matrix: Vec<Vec<i32>> = create_augmented_matrix(&n);
    show_augmented_matrix(&matrix, &n);
    println!("===============================================================");
    println!("===============================================================");
    solve_augmented_matrix(&mut matrix, n);
    show_augmented_matrix(&matrix, &n);
}

fn create_augmented_matrix(n: &i32) -> Vec<Vec<i32>> {
    // Create the augmented matrix as a Vector of Vectors
    let mut augmented_matrix: Vec<Vec<i32>> = Vec::new();

    // Iterate over the matrix and ask user input
    // for coefficients and the constant of each equation
    for i in 0..*n {
        println!("Equation number:{}", i + 1);

        //Create an inner vector
        let mut inner_vector: Vec<i32> = Vec::new();

        for j in 0..=*n {
            println!("Please enter the coeficient for the variable {}", j + 1);
            let coefficient: i32 = get_i32_from_user();

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

fn show_augmented_matrix(matrix: &Vec<Vec<i32>>, n: &i32) {
    for i in 0..*n {
        for j in 0..=*n {
            print!("[{}],", matrix[i as usize][j as usize]);
        }
        println!();
    }
}

fn solve_augmented_matrix(matrix: &mut Vec<Vec<i32>>, n: i32) {
    //set the pivot column for each column on the left side of the matrix
    for j in 0..n {
        //check if the value at [j][j](main diagonal) is different from 0
        // if it its, set it as the pivot element
        if matrix[j as usize][j as usize] != 0 {
            // Apply a division through the whole row, to make the pivot element equal to 1
            let pivot_value: i32 = matrix[j as usize][j as usize];
            for j2 in 0..=n {
                matrix[j as usize][j2 as usize] /= pivot_value;
            }
        }
    }
}
