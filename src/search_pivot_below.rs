use crate::swap_rows;

pub fn search_pivot_below(n: i32, pivot_index: i32, matrix: &mut Vec<Vec<f64>>) {
    for i in pivot_index + 1..n {
        if matrix[i as usize][pivot_index as usize] != 0.0 {
            swap_rows::swap_rows(matrix, pivot_index, i, n);
            break;
        }
    }
}
