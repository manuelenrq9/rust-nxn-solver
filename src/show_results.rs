use crate::format_zero::format_zero;

pub fn show_results(matrix: Vec<Vec<f64>>, n: i32) {
    let mut value: f64;
    let mut first_variable: bool;
    let mut coeficient: String;
    for i in 0..n {
        first_variable = true;
        for j in 0..n {
            value = matrix[i as usize][j as usize];
            if value != 0.0 {
                if !first_variable && value > 0.0 {
                    print!("+ ");
                };

                if value == 1.0 {
                    coeficient = "".to_string();
                } else if value == -1.0 {
                    coeficient = "- ".to_string();
                } else {
                    coeficient = value.to_string();
                }

                print!("{}x{} ", coeficient, j + 1);
                first_variable = false;
            }
        }
        if !first_variable {
            println!(" = {:.2}", format_zero(matrix[i as usize][n as usize]));
        }
    }
}
