fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

fn main() {
    let temp_f = 100.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F is {temp_c}°C");

    let temp_c = 0.0;
    let temp_f = celsius_to_fahrenheit(temp_c);
    println!("{temp_c}°C is {temp_f}°F");
}