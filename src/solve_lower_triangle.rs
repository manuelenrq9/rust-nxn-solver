use crate::check_logical_contradiction;
use crate::pivot_row_division;
use crate::search_pivot_below;
use crate::search_pivot_right;
use crate::solve_lower_column;

pub fn solve_lower_triangle(matrix: &mut Vec<Vec<f64>>, n: i32) {
    for pivot_index in 0..n {
        //first check if the current row has any mathematical contradiction
        check_logical_contradiction::check_logical_contradiction(matrix, pivot_index, n);
        //check if the value at [pivot_index][pivot_index] (main diagonal) is different from 0
        // if it its, set it as the pivot element, else, swap the row with another one below it
        let mut pivot_value: f64 = matrix[pivot_index as usize][pivot_index as usize];
        if pivot_value == 0.0 {
            if search_pivot_below::search_pivot_below(n, pivot_index, matrix) {
                //set the new pivot value
                pivot_value = matrix[pivot_index as usize][pivot_index as usize];
            }
        }
        if pivot_value == 0.0 {
            //if there are no values different from zero below, search the closest one to the right
            //and set it as the pivot on a row of the same index by swapping the rows.
            if search_pivot_right::search_pivot_right(matrix, pivot_index, n) {
                check_logical_contradiction::check_logical_contradiction(matrix, pivot_index, n);
            }
        } else {
            // Apply a division through the whole row, to make the pivot element equal to 1
            pivot_row_division::pivot_row_division(n, pivot_index, pivot_value, matrix);
            //solve lower column values
            solve_lower_column::solve_lower_column(n, pivot_index, matrix);
        }
    }
}
