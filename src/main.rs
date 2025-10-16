mod convert_string_to_f64;
mod convert_string_to_i32;
mod create_augmented_matrix;
mod get_f64_from_user;
mod get_i32_from_user;
mod pivot_row_division;
mod search_pivot_below;
mod search_pivot_right;
mod show_augmented_matrix;
mod solve_augmented_matrix;
mod solve_lower_column;
mod solve_lower_triangle;
mod solve_upper_column;
mod solve_upper_triangle;
mod swap_rows;

fn main() {
    println!("Welcome to the NxN calculator for systems of linear equations");
    println!("Please write down the value for N: ");
    let n: i32 = get_i32_from_user::get_i32_from_user();
    let mut matrix: Vec<Vec<f64>> = create_augmented_matrix::create_augmented_matrix(&n);
    show_augmented_matrix::show_augmented_matrix(&matrix, &n);
    println!("===============================================================");
    println!("===============================================================");
    solve_augmented_matrix::solve_augmented_matrix(&mut matrix, n);
    show_augmented_matrix::show_augmented_matrix(&matrix, &n);
}
