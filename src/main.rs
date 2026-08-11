use std::io;
fn main() {
    let first_number = 10;
    let second_number = 5;

    let addition = first_number + second_number;
    let subtraction = first_number - second_number;
    let multiplication = first_number * second_number;
    let division = first_number / second_number;

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
