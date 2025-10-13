use crate::pivot_row_division;
use crate::show_augmented_matrix;
use crate::solve_lower_column;
use crate::swap_rows;

pub fn solve_lower_triangle(matrix: &mut Vec<Vec<f64>>, n: i32) {
    for pivot_index in 0..n {
        //check if the value at [pivot_index][pivot_index] (main diagonal) is different from 0
        // if it its, set it as the pivot element, else, swap the row with another one below it
        let mut pivot_value: f64 = matrix[pivot_index as usize][pivot_index as usize];
        if pivot_value == 0.0 {
            swap_rows::swap_rows(n, pivot_index, matrix);
            pivot_value = matrix[pivot_index as usize][pivot_index as usize];
        }
        // Apply a division through the whole row, to make the pivot element equal to 1
        pivot_row_division::pivot_row_division(n, pivot_index, pivot_value, matrix); //problem here, carefull with division by zero. 
        show_augmented_matrix::show_augmented_matrix(matrix, &n);
        println!("===============================================================");
        println!("===============================================================");
        //solve lower column values
        solve_lower_column::solve_lower_column(n, pivot_index, matrix, pivot_value);
        show_augmented_matrix::show_augmented_matrix(matrix, &n);
        println!("===============================================================");
        println!("===============================================================");
    }
}
