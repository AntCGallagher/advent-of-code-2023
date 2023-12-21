use std::env;
use std::fs;

fn main() {
    // Check command line arguments
    let argc: Vec<String> = env::args().collect();
    let argv: usize = argc.len();
    if argv < 2 {
        println!("Usage: input_file num_red num_green num_blue");
        return;
    }

    // Read input file
    let input_file: &String = &argc[1];
    let contents: String = match fs::read_to_string(input_file) {
        Err(e) => {
            eprintln!("Error reading input_file {}: {}", input_file, e);
            return;
        }
        Ok(f) => f,
    };

    let result: usize = match sum_part_numbers_from_engine_schematic(&contents) {
        Ok(result) => result,
        Err(message) => {
            eprintln!("Error calculating sum of part numbers: {}", message);
            return;
        }
    };

    println!("{}", result);
}

fn sum_part_numbers_from_engine_schematic(engine_schematic: &String) -> Result<usize, String> {
    // Parse through the lines of the schematic
    let mut parts: Vec<EnginePart> = Vec::new();
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    let mut parsing_state: Option<PartNumberParsingState> = None;

    for (y, line) in engine_schematic.split("\n").enumerate() {
        for (x, character) in line.chars().enumerate() {
            // If we've just stopped parsing a character, parse the result and commit.
            if !is_digit(character) && parsing_state.is_some() {
                match parsing_state {
                    Some(existing) => {
                        match from_parsing_state(&existing, x, y) {
                            Ok(result) => part_numbers.push(result),
                            Err(e) => return Err(e),
                        }
                    }
                    None => {}
                };
                parsing_state = None;
            }

            if is_engine_part(character) {
                parts.push(EnginePart {
                    value: character,
                    x_loc: x,
                    y_loc: y,
                });
            } else if is_digit(character) {
                match parsing_state {
                    Some(ref mut existing) => existing.part_number_digits.push(character),
                    None => {
                        parsing_state = Some(PartNumberParsingState {
                            parsing_part_number_x_start: x,
                            part_number_digits: vec![character],
                        })
                    }
                };
            }
        }

        // If we're still parsing a part number at the end of the line, parse the result and commit.
        if parsing_state.is_some() {
            match parsing_state {
                Some(existing) => match from_parsing_state(&existing, line.chars().count(), y) {
                    Ok(result) => part_numbers.push(result),
                    Err(e) => return Err(e),
                },
                None => {}
            };
            parsing_state = None;
        };
    }

    // For each gear, calculate the gear ratio and add to sum
    let mut sum: usize = 0;

    for part in parts {
        if part.value != '*' {
            continue;
        }

        let mut associated_part_numbers: Vec<usize> = Vec::new();

        for part_number in &part_numbers {
            if is_part_number_for_part(&part_number, &part) {
                associated_part_numbers.push(part_number.value);
            }    
        }

        if associated_part_numbers.iter().count() == 2 {
            let gear_ratio: usize = associated_part_numbers.get(0).unwrap() * associated_part_numbers.get(1).unwrap(); 
            sum += gear_ratio;
        }
    }

    return Ok(sum);
}

fn from_parsing_state(
    parsing_state: &PartNumberParsingState,
    current_x: usize,
    current_y: usize,
) -> Result<PartNumber, String> {
    let value = match format!(
        "{}",
        parsing_state.part_number_digits.iter().collect::<String>()
    )
    .parse::<usize>()
    {
        Ok(value) => value,
        Err(e) => return Err(format!("Error parsing part number: {}", e)),
    };

    return Ok(PartNumber {
        value,
        x_start_loc: parsing_state.parsing_part_number_x_start,
        x_end_loc: current_x,
        y_loc: current_y,
    });
}

fn is_digit(character: char) -> bool {
    return character >= '0' && character <= '9';
}

fn is_engine_part(character: char) -> bool {
    return !is_digit(character) && character != '.';
}

fn is_part_number_for_part(part_number: &PartNumber, part: &EnginePart) -> bool {
    // A part number is associated to a part if any part of it is adjacent (including diagonally) to the part.

    // Check |part_number.y - part.y| <= 1
    if (part_number.y_loc as i32 - part.y_loc as i32).abs() > 1 {
        return false;
    }

    // Check |part_number.x - part.x| <= 1 for at least one x in part_number
    for x in part_number.x_start_loc..part_number.x_end_loc {
        if (x as i32 - part.x_loc as i32).abs() <= 1 {
            return true;
        }
    }

    return false;
}

struct PartNumberParsingState {
    part_number_digits: Vec<char>,
    parsing_part_number_x_start: usize,
}

// An engine part will be a non-numeric, non-period symbol, and an associated x- and y-coordinate.
struct EnginePart {
    value: char,
    x_loc: usize,
    y_loc: usize,
}

// A part number can span across multiple x values, but only one y value
struct PartNumber {
    value: usize,
    x_start_loc: usize,
    x_end_loc: usize,
    y_loc: usize,
}

#[cfg(test)]
mod tests {

    use std::fs;

    #[test]
    fn test() {
        // Read input file
        let input_file: String = "res/test_input.txt".to_string();
        let contents: String = match fs::read_to_string(input_file) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        let result = match crate::sum_part_numbers_from_engine_schematic(&contents) {
            Ok(res) => res,
            Err(message) => panic!("{}", message),
        };
        assert_eq!(result, 467835);
    }
}
