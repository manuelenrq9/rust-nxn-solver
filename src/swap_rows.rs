pub fn swap_rows(matrix: &mut Vec<Vec<f64>>, pivot_index: i32, i: i32, n: i32) {
    let mut aux: f64;
    for j in 0..=n {
        aux = matrix[pivot_index as usize][j as usize];
        matrix[pivot_index as usize][j as usize] = matrix[i as usize][j as usize];
        matrix[i as usize][j as usize] = aux;
    }
}
