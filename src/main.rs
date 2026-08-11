use std::io;

// Addition function
fn add(a: f64, b: f64) -> f64 {
    a + b
}

// subtraction function
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

//Multiplication function
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

// Division function
fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    let first_number = 10.0;
    let second_number = 5.0;

    let addition = add(first_number, second_number);
    let subtraction = subtract(first_number, second_number);
    let multiplication = multiply(first_number, second_number);
    let division = divide(first_number, second_number);

    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);

    println!("Addition: {}", addition);
    println!("Subtraction: {}", subtraction);
    println!("Multiplication: {}", multiplication);
    println!("Division: {}", division);

    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: i32 = input.trim().parse().expect("Please enter a number");

    println!("You entered: {}", number);
}
