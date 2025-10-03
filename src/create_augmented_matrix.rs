use crate::get_f64_from_user;

pub fn create_augmented_matrix(n: &i32) -> Vec<Vec<f64>> {
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
            let coefficient: f64 = get_f64_from_user::get_f64_from_user();

            //assign the coefficient value into the inner vector
            inner_vector.push(coefficient);
        }
        //insert the inner vector (equation) into the matrix
        augmented_matrix.push(inner_vector);
    }
    return augmented_matrix;
}
