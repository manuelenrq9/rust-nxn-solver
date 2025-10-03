pub fn solve_augmented_matrix(matrix: &mut Vec<Vec<f64>>, n: i32) {
    //first we solve the lower triangle by doing forward elimination
    //set the pivot column for each column on the left side of the matrix
    for k in 0..n {
        //check if the value at [k][k](main diagonal) is different from 0
        // if it its, set it as the pivot element
        let pivot_value: f64 = matrix[k as usize][k as usize];
        if pivot_value != 0.0 {
            // Apply a division through the whole row, to make the pivot element equal to 1
            for j in 0..=n {
                matrix[k as usize][j as usize] /= pivot_value;
            }
            //make all values below the pivot element equal to zero,
            //by substracting the pivot element's row multiplied by
            //the value you wanto to turn to zero
            for i in k + 1..n {
                let row_multiplier: f64 = matrix[i as usize][k as usize];
                for j in 0..=n {
                    let substracting_value: f64 = matrix[k as usize][j as usize] * row_multiplier;
                    matrix[i as usize][j as usize] -= substracting_value;
                }
            }
        }
    }
    //backward elimination for solving the upper triangle
    for k in (0..n).rev() {
        //check if the value at [k][k](main diagonal) is different from 0
        // if it its, set it as the pivot element
        let pivot_value: f64 = matrix[k as usize][k as usize];
        if pivot_value != 0.0 {
            //make all values above the pivot element equal to zero,
            //by substracting the pivot element's row multiplied by
            //the value you wanto to turn to zero
            for i in (0..k).rev() {
                let row_multiplier: f64 = matrix[i as usize][k as usize];
                for j in 0..=n {
                    let substracting_value: f64 = matrix[k as usize][j as usize] * row_multiplier;
                    matrix[i as usize][j as usize] -= substracting_value;
                }
            }
        }
    }
}
