pub fn convert_string_to_f64(s: String) -> f64 {
    let n: f64 = s
        .trim()
        .parse()
        .expect("Failed to parse number: Not a valid integer");

    return n;
}
