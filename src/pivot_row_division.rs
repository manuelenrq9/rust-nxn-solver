pub fn pivot_row_division(n: i32, pivot_index: i32, pivot_value: f64, matrix: &mut Vec<Vec<f64>>) {
    //Apply a division through the whole row, to make the pivot element equal to 1
    if pivot_value != 0.0 {
        for j in 0..=n {
            matrix[pivot_index as usize][j as usize] /= pivot_value;
        }
    }
}
