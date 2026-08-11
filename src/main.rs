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
    println!("==Rust Calculator==");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Exit");

    println!("Choose an operation:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    println!("Enter first number:");

    let mut first_input = String::new();

    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read input");

    let first_number: f64 = first_input.trim().parse().expect("Please enter a number");

    println!("Enter second number:");

    let mut second_input = String::new();

    io::stdin()
        .read_line(&mut second_input)
        .expect("Failed to read input");

    let second_number: f64 = second_input.trim().parse().expect("Please enter a number");

    match choice {
        1 => println!("Result: {}", add(first_number, second_number)),

        2 => println!("Result: {}", subtract(first_number, second_number)),

        3 => println!("Result: {}", multiply(first_number, second_number)),

        4 => println!("Result: {}", divide(first_number, second_number)),

        5 => println!("Goodbye",),

        _ => println!("Invalid choice"),
    }
}
