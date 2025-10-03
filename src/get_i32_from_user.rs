use crate::convert_string_to_i32;
use std::io;

pub fn get_i32_from_user() -> i32 {
    let mut s: String = String::new();

    //get user input as a String and store it in s
    io::stdin().read_line(&mut s).expect("Failed to read line");

    // Convert the user input (String) into an f64 value
    let n: i32 = convert_string_to_i32::convert_string_to_i32(s);

    return n;
}
