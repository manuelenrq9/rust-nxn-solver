pub fn swap_rows(n: i32, pivot_index: i32, matrix: &mut Vec<Vec<f64>>) {
    let mut aux: f64;
    for i in pivot_index + 1..n - 1 {
        if matrix[i as usize][pivot_index as usize] != 0.0 {
            for j in 0..n {
                aux = matrix[pivot_index as usize][j as usize];
                matrix[pivot_index as usize][j as usize] = matrix[i as usize][j as usize];
                matrix[i as usize][j as usize] = aux;
            }
            break;
        }
    }
}
