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
    if b == 0.0 {
        println!("Error: Cannot divide by zero.");
        0.0
    } else {
        a / b
    }
}

// Get a number from the user
fn get_number(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    loop {
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

        match choice {
            1 => {
                let first_number = get_number("Enter first number:");
                let second_number = get_number("Enter second number:");

                println!("Result: {}", add(first_number, second_number));
            }

            2 => {
                let first_number = get_number("Enter first number:");
                let second_number = get_number("Enter second number:");

                println!("Result: {}", subtract(first_number, second_number));
            }

            3 => {
                let first_number = get_number("Enter first number:");
                let second_number = get_number("Enter second number:");

                println!("Result: {}", multiply(first_number, second_number));
            }

            4 => {
                let first_number = get_number("Enter first number:");
                let second_number = get_number("Enter second number:");

                println!("Result: {}", divide(first_number, second_number));
            }

            5 => {
                println!("Goodbye");
                break; // This stop the loop from continiously executing
            }

            _ => {
                println!("Invalid choice");
            }
        }
    }
}
