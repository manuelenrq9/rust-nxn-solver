use crate::pivot_row_division;
use crate::solve_lower_column;
use crate::swap_pivot;

pub fn solve_lower_triangle(matrix: &mut Vec<Vec<f64>>, n: i32) {
    for pivot_index in 0..n {
        //check if the value at [pivot_index][pivot_index] (main diagonal) is different from 0
        // if it its, set it as the pivot element, else, swap the row with another one below it

        let mut pivot_value: f64 = matrix[pivot_index as usize][pivot_index as usize];
        if pivot_value == 0.0 {
            swap_pivot::swap_pivot(n, pivot_index, matrix);
            pivot_value = matrix[pivot_index as usize][pivot_index as usize];
            if pivot_value == 0.0 {}
        }
        // Apply a division through the whole row, to make the pivot element equal to 1
        pivot_row_division::pivot_row_division(n, pivot_index, pivot_value, matrix);
        //solve lower column values
        solve_lower_column::solve_lower_column(n, pivot_index, matrix);
    }
}
