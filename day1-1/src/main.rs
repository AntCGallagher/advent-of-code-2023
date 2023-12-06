use std::env;
use std::fs;

// Takes an input file as command line argument.
// For each line:
// - Finds the first and last numeric digits (ASCII '0' - '9').
// - Combines these digits into a two digit decimal number.
// - Sums all of the constructed numbers.
// - Prints the result to stdout.
fn main() {
    // Check command line arguments
    let argc: Vec<String> = env::args().collect();
    let argv: usize = argc.len();
    if argv < 2 {
        println!("Usage: input_file");
        return;
    }

    // Read input file
    let input_file: &String = &argc[1];
    let contents: String = match fs::read_to_string(input_file) {
        Err(e) => {
            eprintln!("Error reading input_file {}: {}", input_file, e);
            return;
        },
        Ok(f) => f,
    };

    // Iterate through every line
    let mut result: i32 = 0; 
    for line in contents.split("\n") {
        // Ignore empty lines
        if line.is_empty() {
            continue;
        }
        // Find first digit and last digit
        // Note: If only one digit is present on the line, last digit will be the same as first digit
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;
        for c in line.chars() {
            if is_digit(c) {
                if first_digit.is_none() {
                    first_digit = Some(c);
                    last_digit = Some(c);
                } else {
                    last_digit = Some(c);
                }
            }
        }
        // If we don't have any digits on the line, print error message
        if first_digit.is_none() || last_digit.is_none() {
            eprintln!("Malformed input. Line {} does not contain any numeric characters.", line);
            return;
        }
        // Parse digits as integer, add to cumulative result
        result += format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap();
    }

    // Print result
    println!("{}", result);
    return;
}

// Returns true if the supplied character is a digit (ASCII '0' - '9'), false otherwise.
fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}