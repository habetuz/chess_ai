use std::io::Write;

use crate::engine::GameState;
use crate::engine::GameState::Normal;

pub fn turn(board: [[char; 8]; 8], white: bool) -> GameState {
    let input = loop {
        if white {
            print!("White> ");
        } else {
            print!("Black> ");
        }

        std::io::stdout().flush().expect("Could not flush stdout!");

        let input = {
            let mut input = "".to_string();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");
            input
        };

        let input = input.to_uppercase();

        match parse_input(input) {
            Ok(value) => {
                break value;
            }
            Err(_) => {
                println!("Could not read input!");
                continue;
            }
        }
    };

    println!("Move {} to {}", input.0, input.1);

    if white {
    } else {
    }

    Normal(board)
}

fn parse_input<'a>(input: String) -> Result<(String, String), ()> {
    let input: Vec<&str> = input.split_whitespace().collect();

    if input.len() != 2
        || !validate_coordinate(input[0])
        || !validate_coordinate(input[1])
        || input[0] == input[1]
    {
        return Err(());
    }

    Ok((input[0].chars().collect(), input[1].chars().collect()))
}

fn validate_coordinate(coordinate: &str) -> bool {
    let coordinate: Vec<char> = coordinate.chars().collect();

    coordinate.len() == 2
        && (coordinate[0] >= 'A' && coordinate[0] <= 'H')
        && (coordinate[1] >= '1' && coordinate[1] <= '8')
}
