use crate::show_augmented_matrix;
use crate::solve_upper_column;

pub fn solve_upper_triangle(matrix: &mut Vec<Vec<f64>>, n: i32) {
    for pivot_index in (0..n).rev() {
        //solve upper triangle
        solve_upper_column::solve_upper_column(n, pivot_index, matrix);
        show_augmented_matrix::show_augmented_matrix(matrix, &n);
        println!("===============================================================");
        println!("===============================================================");
    }
}
