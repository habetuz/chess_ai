use crate::engine::*;
use crate::GameState::Normal;
use colored::*;
use std::io::Write;

const PADDING: char = ' ';
const EMPTY_FIELD: char = ' ';

pub fn turn(board: Board, white: bool) -> GameState {
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

pub fn print_board(board: Board) {
    println!("    A B C D E F G H");
    println!("  ╔════════════════╗");
    for y in (1..=8).rev() {
        if y % 2 == 0 {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                get_str(board, 1, y).on_white(),
                get_str(board, 2, y).on_bright_black(),
                get_str(board, 3, y).on_white(),
                get_str(board, 4, y).on_bright_black(),
                get_str(board, 5, y).on_white(),
                get_str(board, 6, y).on_bright_black(),
                get_str(board, 7, y).on_white(),
                get_str(board, 8, y).on_bright_black(),
                PADDING.to_string().on_white(),
                PADDING.to_string().on_bright_black()
            );
        } else {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                get_str(board, 1, y).on_bright_black(),
                get_str(board, 2, y).on_white(),
                get_str(board, 3, y).on_bright_black(),
                get_str(board, 4, y).on_white(),
                get_str(board, 5, y).on_bright_black(),
                get_str(board, 6, y).on_white(),
                get_str(board, 7, y).on_bright_black(),
                get_str(board, 8, y).on_white(),
                PADDING.to_string().on_bright_black(),
                PADDING.to_string().on_white()
            );
        }
    }
    println!("  ╚════════════════╝");
    println!("    A B C D E F G H");
}

//! Get a char representing either the figure at the given position or a space.
fn get_str(board: Board, x: u8, y: u8) -> ColoredString {
    for piece in board {
        if x == piece.1 && y == piece.2 {
            return piece.0.to_string().black();
        }
    }
    EMPTY_FIELD.to_string().black()
}
