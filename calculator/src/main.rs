use std::io;
use regex::Regex;

fn calculate(input: &str) -> Result<f64, &'static str> {
    let re = Regex::new(r"([\d.]+)\s*([+-/*])\s*([\d.]+)").expect("Invalid regex");

    let captures = re.captures(input).ok_or("Invalid input format")?;

    let num1: f64 = captures[1].parse().map_err(|_| "Invalid number")?;

    let num2: f64 = captures[3].parse().map_err(|_| "Invalid number")?;

    let op = captures[2].chars().next().ok_or("Invalid operator")?;

    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => (num2.abs() > 0.0).then(|| num1 / num2).ok_or("Cannot divide by zero"),
        _ => Err("Unsupported operator"),
    }

}

fn main() {
    println!("Welcome to the calculator!");

    loop {
        println!("Enter an expression to calculate (e.g., 2 + 2), or type 'exit' to quit.");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let input = input.trim();

        if input.eq("exit") {
            println!("Goodbye!");
            break;
        }

        let result = calculate(input).unwrap_or_else(|err| {
            println!("Error: {}", err);
            0.0
        });

        println!("{}", result);
    }

}
