pub fn solve_upper_column(n: i32, pivot_index: i32, matrix: &mut Vec<Vec<f64>>) {
    //make all values above the pivot element equal to zero,
    //by substracting the pivot element's row multiplied by
    //the value you wanto to turn to zero
    for i in (0..pivot_index).rev() {
        let row_multiplier: f64 = matrix[i as usize][pivot_index as usize];
        for j in 0..=n {
            let substracting_value: f64 = matrix[pivot_index as usize][j as usize] * row_multiplier;
            matrix[i as usize][j as usize] -= substracting_value;
        }
    }
}
