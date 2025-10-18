use crate::epsilon_check;

pub fn show_augmented_matrix(matrix: &Vec<Vec<f64>>, n: i32) {
    for i in 0..n {
        for j in 0..=n {
            print!(
                "[{}],",
                epsilon_check::epsilon_check(matrix[i as usize][j as usize])
            );
        }
        println!();
    }
    println!("===============================================================");
    println!("===============================================================");
}
