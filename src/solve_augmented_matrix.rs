use crate::{solve_lower_triangle, solve_upper_triangle};

pub fn solve_augmented_matrix(matrix: &mut Vec<Vec<f64>>, n: i32) {
    //forward elimination for solving the lower triangle
    solve_lower_triangle::solve_lower_triangle(matrix, n);
    //backward elimination for solving the upper triangle
    solve_upper_triangle::solve_upper_triangle(matrix, n);
}
