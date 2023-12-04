use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}


fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                println!("Cannot divide by zero.");
                0.0
            } else {
                a / b
            }
        },
    }
}


fn main() {
    println!("Welcome to the Calculator CLI")
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Please type a number!");
    input.clear();

    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim();
    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Please type a number!");

    let op = match operation.to_lowercase().as_str() {
        "add" => Operation::Add(first_number, second_number),
        "subtract" => Operation::Subtract(first_number, second_number),
        "multiply" => Operation::Multiply(first_number, second_number),
        "divide" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(op);
    println!("Result: {}", result);
}