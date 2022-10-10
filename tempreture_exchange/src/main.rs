use std::io;

const DEGREE_CELSIUS: char = '0';
const DEGREE_FAHRENHEIT: char = '1';

fn main() {
    println!("Do you want to exchange celsius(0) or fahrenheit(1):");
    let mut condition = String::new();
    io::stdin()
        .read_line(&mut condition)
        .expect("Failed to read line.");
    let condition: char = match condition.trim()
        .parse() {
        Ok(x) => x,
        Err(_) => '0',
    };

    println!("Enter your degree:");
    let mut degree = String::new();
    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read line.");
    let degree: f64 = match degree.trim()
        .parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };

    match condition {
        DEGREE_CELSIUS => {
            println!("The celsius degree exchanging to fahrenheit degree is {}.", 32.0 + degree * 1.8);
        }
        DEGREE_FAHRENHEIT => {
            println!("The celsius fahrenheit exchanging to degree degree is {}.", (degree - 32.0) / 1.8);
        }
        _ => {
            println!("error.");
        }
    }
}
