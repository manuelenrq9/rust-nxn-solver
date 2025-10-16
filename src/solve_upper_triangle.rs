use crate::solve_upper_column;

pub fn solve_upper_triangle(matrix: &mut Vec<Vec<f64>>, n: i32) {
    for pivot_index in (0..n).rev() {
        //solve upper triangle
        let pivot_value: f64 = matrix[pivot_index as usize][pivot_index as usize];
        if pivot_value != 0.0 {
            solve_upper_column::solve_upper_column(n, pivot_index, matrix);
        }
    }
}
