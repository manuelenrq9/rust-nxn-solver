use crate::convert_string_to_f64;
use std::io;

pub fn get_f64_from_user() -> f64 {
    let mut s: String = String::new();

    //get user input as a String and store it in s
    io::stdin().read_line(&mut s).expect("Failed to read line");

    // Convert the user input (String) into an f64 value
    let n: f64 = convert_string_to_f64::convert_string_to_f64(s);

    return n;
}
