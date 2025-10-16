use crate::swap_rows;

pub fn search_pivot_right(matrix: &mut Vec<Vec<f64>>, pivot_index: i32, n: i32) {
    for j in pivot_index + 1..n {
        if matrix[pivot_index as usize][j as usize] != 0.0 {
            swap_rows::swap_rows(matrix, pivot_index, j, n);
            break;
        }
    }
}
