// use clap::{App, Arg}; old
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Calculator")
        .version("1.0")
        .author("kw")
        .about("Performs basic arithmetic")
        .arg(
            Arg::new("operation")
                .help("The operation to perform; format should be '<num1> <operator> <num2>'")
                .required(true)
                .index(1),
        )
        .get_matches();

    let operation = matches.get_one::<String>("operation").unwrap();

    match calculate(&operation) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}


//old
// fn main() {
//
//     let matches = App::new("Calculator")
//         .version("1.0")
//         .author("kw")
//         .about("Performs basic arithmetic")
//         .arg(Arg::with_name("operation")
//             .help("The operation to perform; format should be '<num1> <operator> <num2>'")
//                  .required(true)
//                  .index(1))
//         .get_matches();
//
//     let operation = matches.value_of("operation").unwrap();
//
//     match calculate(operation) {
//         Ok(result) => println!("Result: {}", result),
//         Err(err) => println!("Error: {}", err),
//     }
// }

fn calculate(input: &str) -> Result<f64, &'static str> {

    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 3 {
        return Err("Input must be a binary operation like '2 + 2'");
    }

    let num1 = tokens[0].parse::<f64>().map_err(|_| "Invalid number")?;

    let operator = tokens[1];

    let num2 = tokens[2].parse::<f64>().map_err(|_| "Invalid number")?;

    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => if num2 != 0.0 { Ok(num1/num2) } else { Err("Cannot divide by zero") },
        _ => Err("Unknown operator"),
    }

}

// cargo run -- "10 / 2"