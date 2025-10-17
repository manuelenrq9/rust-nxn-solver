use crate::pivot_row_division;
use crate::search_pivot_below;
use crate::search_pivot_right;
use crate::show_augmented_matrix;
use crate::solve_lower_column;

pub fn solve_lower_triangle(matrix: &mut Vec<Vec<f64>>, n: i32) {
    for pivot_index in 0..n {
        //check if the value at [pivot_index][pivot_index] (main diagonal) is different from 0
        // if it its, set it as the pivot element, else, swap the row with another one below it
        let mut pivot_value: f64 = matrix[pivot_index as usize][pivot_index as usize];
        if pivot_value == 0.0 {
            println!("pivote:{}, indice:{} ", pivot_value, pivot_index);
            show_augmented_matrix::show_augmented_matrix(matrix, &n);
            search_pivot_below::search_pivot_below(n, pivot_index, matrix);
            //set the new pivot value
            pivot_value = matrix[pivot_index as usize][pivot_index as usize];
            println!("pivote:{}", pivot_value);
            show_augmented_matrix::show_augmented_matrix(matrix, &n);
        }
        if pivot_value == 0.0 {
            //if there are no values different from zero below, search the closest one to the right
            //and set it as the pivot on a row of the same index by swapping the rows.
            search_pivot_right::search_pivot_right(matrix, pivot_index, n);
        } else {
            // Apply a division through the whole row, to make the pivot element equal to 1
            pivot_row_division::pivot_row_division(n, pivot_index, pivot_value, matrix);
            //solve lower column values
            solve_lower_column::solve_lower_column(n, pivot_index, matrix);
        }
    }
}
