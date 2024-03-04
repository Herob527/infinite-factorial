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

    let mut calculation = Calculation {
        first_row: vec![1],
        second_row: vec![1],
    };
    for i in 1..(string_number + 2) {
        let result = calculation.multiply();
        calculation.first_row = result;
        calculation.second_row = vec![i]
    }
    let stringified_row = calculation
        .first_row
        .into_iter()
        .map(|el| el.to_string())
        .collect::<String>();
    stringified_row
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
    Ok(string_number)
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
}

#[cfg(test)]

mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_factorial_0() {
        debug_assert_eq!(calculate_factorial(0), "1")
    }
    #[test]
    fn test_calculate_factorial_1() {
        debug_assert_eq!(calculate_factorial(1), "1")
    }
    #[test]
    fn test_calculate_factorial_2() {
        debug_assert_eq!(calculate_factorial(2), "2")
    }
    #[test]
    fn test_calculate_factorial_3() {
        debug_assert_eq!(calculate_factorial(3), "6")
    }
    #[test]
    fn test_calculate_factorial_4() {
        debug_assert_eq!(calculate_factorial(4), "24")
    }
    #[test]
    fn test_calculate_factorial_5() {
        debug_assert_eq!(calculate_factorial(5), "120")
    }
    #[test]
    fn test_calculate_factorial_6() {
        debug_assert_eq!(calculate_factorial(6), "720")
    }
    #[test]
    fn test_calculate_factorial_7() {
        debug_assert_eq!(calculate_factorial(7), "5040")
    }
    #[test]
    fn test_calculate_factorial_8() {
        debug_assert_eq!(calculate_factorial(8), "40320")
    }
    #[test]
    fn test_calculate_factorial_9() {
        debug_assert_eq!(calculate_factorial(9), "362880")
    }
    #[test]
    fn test_calculate_factorial_10() {
        debug_assert_eq!(calculate_factorial(10), "3628800")
    }
    #[test]
    fn test_calculate_factorial_11() {
        debug_assert_eq!(calculate_factorial(11), "39916800")
    }
    #[test]
    fn test_calculate_factorial_12() {
        debug_assert_eq!(calculate_factorial(12), "479001600")
    }
    #[test]
    fn test_calculate_factorial_13() {
        debug_assert_eq!(calculate_factorial(13), "6227020800")
    }
    #[test]
    fn test_calculate_factorial_14() {
        debug_assert_eq!(calculate_factorial(14), "87178291200")
    }
    #[test]
    fn test_calculate_factorial_15() {
        debug_assert_eq!(calculate_factorial(15), "1307674368000")
    }
    #[test]
    fn test_calculate_factorial_16() {
        debug_assert_eq!(calculate_factorial(16), "20922789888000")
    }
    #[test]
    fn test_calculate_factorial_17() {
        debug_assert_eq!(calculate_factorial(17), "355687428096000")
    }
    #[test]
    fn test_calculate_factorial_18() {
        debug_assert_eq!(calculate_factorial(18), "6402373705728000")
    }
    #[test]
    fn test_calculate_factorial_19() {
        debug_assert_eq!(calculate_factorial(19), "121645100408832000")
    }
    #[test]
    fn test_calculate_factorial_20() {
        debug_assert_eq!(calculate_factorial(20), "2432902008176640000")
    }
    #[test]
    fn test_calculate_factorial_21() {
        debug_assert_eq!(calculate_factorial(21), "51090942171709440000")
    }
}
