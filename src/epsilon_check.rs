pub fn epsilon_check(number: f64) -> f64 {
    let epsilon: f64 = 0.05;
    let nearest_integer: f64 = number.round();
    let difference: f64 = (number - nearest_integer).abs();
    if difference < epsilon {
        return nearest_integer;
    } else {
        return number;
    }
}
