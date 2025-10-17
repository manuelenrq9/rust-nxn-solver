use crate::show_augmented_matrix;
use std::process;

pub fn check_logical_contradiction(matrix: &mut Vec<Vec<f64>>, index: i32, n: i32) {
    let constant_term: f64 = matrix[index as usize][n as usize];
    if constant_term != 0.0 {
        let mut all_coeficients_zero: bool = true;
        for j in 0..n {
            if matrix[index as usize][j as usize] != 0.0 {
                all_coeficients_zero = false;
            }
        }
        if all_coeficients_zero {
            show_augmented_matrix::show_augmented_matrix(matrix, n);
            println!("logical contradiction found at row number {}", index + 1);
            println!("The system of linear equations has no solution");
            process::exit(0);
        }
    }
}
