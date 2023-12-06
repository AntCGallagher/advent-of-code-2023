use std::env;
use std::fs;
use regex::Regex;
use regex::Captures;

fn main() {
    // Check command line arguments
    let argc: Vec<String> = env::args().collect();
    let argv: usize = argc.len();
    if argv < 5 {
        println!("Usage: input_file num_red num_green num_blue");
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

    // Read bag parameters
    let num_red: i32 = match argc[2].parse::<i32>() {
        Err(e) => {
            eprintln!("Error parsing num_red {}: {}", argc[2], e);
            return;
        },
        Ok(result) => result,
    };

    let num_green: i32 = match argc[3].parse::<i32>() {
        Err(e) => {
            eprintln!("Error parsing num_green {}: {}", argc[3], e);
            return;
        },
        Ok(result) => result,
    };

    let num_blue: i32 = match argc[4].parse::<i32>() {
        Err(e) => {
            eprintln!("Error parsing num_blue {}: {}", argc[4], e);
            return;
        },
        Ok(result) => result,
    };

    let bag: Bag = Bag {
        num_red: num_red,
        num_green: num_green,
        num_blue: num_blue,
    };
    
    let sum_of_possible_game_ids = match sum_ids_of_possible_games(&contents, &bag) {
        Err(e) => {
            eprintln!("Error calculating sum of ids: {}", e);
            return;
        },
        Ok(result) => result,
    };

    println!("{}", sum_of_possible_game_ids);
}

pub fn sum_ids_of_possible_games(description: &String, bag: &Bag) -> Result<i32, String> {
    let mut sum_of_possible_game_ids: i32 = 0;
    for line in description.split("\n") {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }
        // Parse games
        let game: Game = match parse_game(&line.to_string()) {
            Err(e) => {
                return Err(format!("Error parsing game {}: {}", line, e));
            },
            Ok(g) => g,
        };
        // Check if possible
        if is_game_possible(&game, bag) {
            sum_of_possible_game_ids += game.id;
        }
    }
    return Ok(sum_of_possible_game_ids);
}

fn parse_game(description: &String) -> Result<Game, String> {
    let game_parts: Vec<&str> = description.split(":").collect();
    if game_parts.len() != 2 {
        return Err(format!("Unexpected number of parts for game {}\nExpected: 2, Actual: {}", description, game_parts.len()));
    }

    let game_header: String = game_parts[0].to_string();
    let game_hands: String = game_parts[1].to_string();

    let id = match parse_game_header(&game_header) {
        Err(e) => {
            return Err(e);
        },
        Ok(result) => result,
    };

    let mut hands: Vec<Hand> = Vec::new();
    for game_hand_string in game_hands.split(";") {
        match parse_hand(&game_hand_string.to_string()) {
            Err(e) => {
                return Err(e);
            },
            Ok(result) => {
                hands.push(result);
            },
        };
    }

    return Ok(Game {
        id: id,
        hands: hands
    });
}

fn parse_game_header(description: &String) -> Result<i32, String> {
    // Format: "Game X" where X is the id of the game
    let game_header_regex: Regex = Regex::new(r"Game ([0-9]+)").unwrap();
    let caps: Captures = match game_header_regex.captures(description) {
        Some(caps) => caps,
        None => {
            return Err("Malformed game header.".to_string());
        },
    };
    // We know it's a valid integer as it matches the regex
    return Ok(caps[1].parse::<i32>().unwrap());
}

fn parse_hand(description: &String) -> Result<Hand, String> {
    let mut num_red: Option<i32> = None;
    let mut num_green: Option<i32> = None;
    let mut num_blue: Option<i32> = None;

    let colour_description_regex: Regex = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();

    for colour_description in description.split(",") {
        let trimmed = colour_description.trim();
        // Check it's at all valid
        let caps: Captures = match colour_description_regex.captures(trimmed) {
            Some(caps) => caps,
            None => {
                return Err(format!("Malformed hand description {}", description)); 
            },
        };
        let num_cubes = caps[1].parse::<i32>().unwrap();
        if trimmed.ends_with("red") {
            if num_red.is_none() {
                num_red = Some(num_cubes);
            } else {
                return Err(format!("Multiple instances of red for hand {}", description));
            }
        }
        if trimmed.ends_with("green") {
            if num_green.is_none() {
                num_green = Some(num_cubes);
            } else {
                return Err(format!("Multiple instances of green for hand {}", description));
            }
        }
        if trimmed.ends_with("blue") {
            if num_blue.is_none() {
                num_blue = Some(num_cubes);
            } else {
                return Err(format!("Multiple instances of blue for hand {}", description));
            }
        }
    }

    return Ok(Hand {
        num_red: num_red.unwrap_or(0),
        num_green: num_green.unwrap_or(0),
        num_blue: num_blue.unwrap_or(0),
    });
}

fn is_game_possible(game: &Game, bag: &Bag) -> bool {
    for hand in &game.hands {
        if !is_hand_possible(&hand, &bag) {
            return false;
        }
    }

    return true;
}

fn is_hand_possible(hand: &Hand, bag: &Bag) -> bool {
    return
        &hand.num_red <= &bag.num_red && 
        &hand.num_green <= &bag.num_green &&
        &hand.num_blue <= &bag.num_blue;
}

pub struct Game {
    id: i32,
    hands: Vec<Hand>,
}

pub struct Bag {
    num_red: i32,
    num_green: i32,
    num_blue: i32,
}

// a given Hand is a subset of the contents of the Bag
type Hand = Bag;

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_GAMES: &str = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const TEST_BAG: Bag = Bag {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };

    #[test]
    fn test() {
        let result = match crate::sum_ids_of_possible_games(&TEST_GAMES.to_string(), &TEST_BAG) {
            Ok(res) => res,
            Err(message) => panic!("{}", message),
        };
        assert_eq!(result, 8);
    }
}