use std::env;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Takes an input file as command line argument.
// For each line:
// - Finds the first and last numeric digits (ASCII '0' - '9') or lowercase digit strings (e.g. "one", "two", "nine").
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

    match collect_and_sum_digits(contents) {
        Ok(result) => println!("{}", result),
        Err(message) => eprintln!("{}", message)
    };

    return;
}

pub fn collect_and_sum_digits(contents: String) -> Result<i32, String> {
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

        // Iterate through each line forwards to find the first digit
        // Then iterate through each line backwards to find the last digit 
        for i in 0..line.len() {
            // End index of slice is exclusive, so add 1 to ensure we cover the full string
            if first_digit.is_none() {
                let forward_substring = &line[0..(i + 1)];
                match as_digit(forward_substring) {
                    Some(digit) => {
                        first_digit = Some(digit);
                    }
                    None => {},
                }    
            }

            // i only goes up to line.len() - 1, so subtract 1 to ensure we cover up to the beginning of the string
            if last_digit.is_none() {
                let reverse_substring = &line[(line.len() - i - 1)..line.len()];
                match as_digit(reverse_substring) {
                    Some(digit) => {
                        last_digit = Some(digit);
                    },
                    None => {},
                }    
            }
            
            if first_digit.is_some() && last_digit.is_some() {
                break;
            }
        }
        // If we don't have any digits on the line, print error message
        if first_digit.is_none() || last_digit.is_none() {
            return Err(format!("Malformed input. Line {} does not contain any numeric characters.", line));
        }
        // Parse digits as integer, add to cumulative result
        result += format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap();
    }

    return Ok(result);
}

// Returns true if the supplied string contains a digit (ASCII '0' - '9'),
// or a lowercase digit string (e.g. "zero", "one", "nine"), returns false otherwise.
fn as_digit(string: &str) -> Option<char> {
    for digit in Digit::iter() {
        if string.contains(digit.name()) || string.contains(&digit.digit_char().to_string())  {
            return Some(*digit.digit_char());
        }
    }

    return None;
}

#[derive(EnumIter)]
enum Digit {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl Digit {
    fn name(&self) -> &'static str {
        match self {
            Digit::ZERO  => "zero",
            Digit::ONE   => "one",
            Digit::TWO   => "two",
            Digit::THREE => "three",
            Digit::FOUR  => "four",
            Digit::FIVE  => "five",
            Digit::SIX   => "six",
            Digit::SEVEN => "seven",
            Digit::EIGHT => "eight",
            Digit::NINE  => "nine",
        }
    }

    fn digit_char(&self) -> &'static char {
        match self {
            Digit::ZERO  => &'0',
            Digit::ONE   => &'1',
            Digit::TWO   => &'2',
            Digit::THREE => &'3',
            Digit::FOUR  => &'4',
            Digit::FIVE  => &'5',
            Digit::SIX   => &'6',
            Digit::SEVEN => &'7',
            Digit::EIGHT => &'8',
            Digit::NINE  => &'9',
        }
    }
}

#[cfg(test)]
mod tests {

    const TEST_VECTOR: &str = "
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test() {
        let result = match crate::collect_and_sum_digits(TEST_VECTOR.to_string()) {
            Ok(res) => res,
            Err(message) => panic!("{}", message),
        };
        assert_eq!(result, 281);
    }
}