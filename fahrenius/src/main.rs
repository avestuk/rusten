use core::panic;
use std::io;

fn main() {
    println!("Please enter the temperature you want converted:\n");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    let temperature_type: String = guess
        .trim()
        .to_ascii_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    println!("temperature_type: {}", temperature_type);

    let is_celsius = match temperature_type.as_str() {
        "C" => true,
        "F" => false,
        _ => panic!(
            "could not match temperature type {} to F or C",
            temperature_type
        ),
    };
    println!("is_celsius: {}", is_celsius);

    let temperature: String = guess.trim().chars().filter(|c| c.is_numeric()).collect();
    println!("guess: {}", temperature);

    let temperature: i32 = temperature.parse().expect("failed to parse:");
    println!("guess: {}", temperature);

    if is_celsius {
        println!("{}F", to_f(temperature))
    } else {
        println!("{}C", to_c(temperature))
    };
}

fn to_c(t: i32) -> f64 {
    (f64::from(t) - 32.0) * (5.0 / 9.0)
}

fn to_f(t: i32) -> f64 {
    (f64::from(t) * (5.0 / 9.0)) + 32.0
}
