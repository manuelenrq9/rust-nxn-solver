pub fn convert_string_to_i32(s: String) -> i32 {
    let n: i32 = s
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    return n;
}
