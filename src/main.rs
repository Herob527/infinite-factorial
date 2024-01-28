use std::env;

use calculation::Calculation;
mod calculation;
mod multiply_vectors;
mod normalise_vector;
mod value;

fn calculate_factorial(string_number: u64) -> String {
    if (0..=1).contains(&string_number) {
        return String::from("1");
    };
    let calculation = Calculation {
        first_row: vec![1, 2, 1],
        second_row: vec![1, 1],
    };
    calculation.multiply();
    return String::from("");
}

fn validate_number(string_number: String) -> Result<String, String> {
    if string_number.starts_with("-") {
        return Err(String::from("Only positive numbers there"));
    };
    for character in string_number.chars() {
        let is_digit = character.is_digit(10);
        if !is_digit {
            let string_format = format!(
                "Provided text '{}' is not a valid decimal number",
                string_number
            );
            return Err(string_format);
        }
    }
    return Ok(string_number);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let actual_arg = &args.get(1);
    if actual_arg.is_none() {
        println!(
            "\x1b[91m[ Error ]\x1b[0m: {}",
            "Required argument wasn't passed. Pass some number like 21 or 37 and we are good."
        );
        return;
    }

    let result = match validate_number(actual_arg.unwrap().to_owned()) {
        Ok(number) => format!(
            "\x1b[92m[ Success ]\x1b[0m: Factorial of number {} is {}",
            number,
            (calculate_factorial(number.parse().unwrap()))
        ),
        Err(string) => format!("\x1b[91m[ Error ]\x1b[0m: {}", string),
    };
    println!("{}", result);
    println!("Actual arg: {}", actual_arg.unwrap());
    dbg!(args);
}
