use crate::format_zero;

pub fn show_augmented_matrix(matrix: &Vec<Vec<f64>>, n: i32) {
    for i in 0..n {
        for j in 0..=n {
            print!(
                "[{:.2}],",
                format_zero::format_zero(matrix[i as usize][j as usize])
            );
        }
        println!();
    }
    println!("===============================================================");
    println!("===============================================================");
}
