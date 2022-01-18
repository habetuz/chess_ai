use crate::engine::figures::*;
use crate::engine::*;
use crate::GameState::{CheckMate, Normal};
use colored::*;
use std::io::Write;

const PADDING: char = ' ';
const EMPTY_FIELD: char = ' ';

/// # Funktion `io::turn`
///
/// Zug-Funktion für Menschen.
///
/// ---
/// ## Parameter
/// `board`: [`Board`], das ein Spielbrett mit Feldern enthält, auf der Figuren stehen können.
///
/// `black_figures`: [`Positions`], die jede schwarze Figur und ihre Position enthält. Figur hat den wert `255` wenn sie geschlagen wurde.
///
/// `black_figures`: [`Positions`], die jede weiße Figur und ihre Position enthält. Figur hat den wert `255` wenn sie geschlagen wurde.
///
/// ---
/// ## Rückgabewert [`GameState`]
/// Gibt entweder einen [`GameState::Normal`] zurück, das das neue Spielbrett enthält, oder einen [`GameState::CheckMate`] wenn der ziehende Spieler gewonnen hat.
pub fn turn(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
) -> GameState {
    // Der Rückgabewert des loops wird in `resulting_board` gespeichert. Siehe [`quick_rust_explanation: Rückgabe von Werten`] wie die Rückgabe von Werten in Rust funktioniert.
    let resulting_board = loop {
        // Der Input wird vom Spieler eingeholt
        let input = loop {
            if white {
                print!("Weiß> ");
            } else {
                print!("Schwarz> ");
            }

            // Terminal macht einen flush normalerweise nur nach einem Zeilenumbruch, da `Weiß>` oder `Schwarz>` schon im Terminal stehen soll, wird hier schon ein flush gemacht.
            std::io::stdout().flush().expect("Could not flush stdout!");

            // Es wird eine Zeile vom Spieler gelesen (bis enter gedrückt wird) und in `input` gespeichert.
            let input = {
                let mut input = "".to_string();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line!");
                input
            };

            // z.B. `a1` wird zu `A1`
            let input = input.to_uppercase();

            // Wenn das parsen erfolgreich war, wird der input in dem Format `(A1, B2)` zurückgegeben.
            // Ansonsten wird dem Spieler mitgeteilt, dass er eine falsche Angabe gemacht hat und der Prozess des Input Einholens beginnt von neuem.
            match parse_input(input) {
                Some(value) => {
                    break value;
                }
                None => {
                    println!("Eingabe ist falsch!");
                    continue;
                }
            }
        };

        // Wenn die zweite Koordinate des Inputs leer ist, bedeutet das, dass der Spieler nur über die möglichen Züge einer Figur bescheid wissen möchte.
        // Ansonsten möchte er eine Figur bewegen.
        if input.1 == "" {
            handle_possible_moves_request(board, black_figures, white_figures, white, input.0);
            // Da der Spieler nun noch keinen Zug gemacht hat, wird der ganze Zug Prozess noch einmal von vorne gestartet.
            continue;
        } else {
            // Wenn der Zug möglich war, wird das neue Spielbrett aus dem loop zurückgegeben.
            // Ansonsten wird eine Error-Nachricht ausgegeben und der Zug Prozess beginnt noch einmal von vorne.
            match handle_move_request(board, black_figures, white_figures, white, input) {
                Ok(value) => {
                    break value;
                }
                Err(value) => {
                    println!("{}", value);
                    continue;
                }
            };
        };
    };

    // Es wird überprüft ob der nicht ziehende Spieler nun Schach Matt ist und in diesem Fall wird ein [`GameState::CheckMate`] zurückgegeben.
    // Ansonsten wird ein normaler status zurückgegeben. Beide Möglichkeiten enthalten das neue Spielfeld.
    if is_checkmate(
        resulting_board.0,
        resulting_board.1,
        resulting_board.2,
        !white,
    ) {
        CheckMate(resulting_board)
    } else {
        Normal(resulting_board)
    }
}

fn handle_possible_moves_request(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
    input: String,
) {
    let mut chars = input.chars();
    let position: (u8, u8) = (
        chars.next().unwrap() as u8 - 'A' as u8 + 1,
        chars.next().unwrap() as u8 - '1' as u8 + 1,
    );

    let figure = get_figure(board, position.0, position.1);
    if figure == 0 {
        println!("There is no figure at that position!");
        return;
    }

    if (white && figure / 10 != 0) || (!white && figure / 10 != 1) {
        println!("The figure you want to move does not belong to you!");
        return;
    }

    let mut moves = get_valid_moves(board, figure, position.0, position.1);

    for i in 0..moves.len() {
        let mut subset = moves[i];
        for i in 0..subset.len() {
            let mut r#move = subset[i];

            if r#move.0 > 8 {
                break;
            }

            let board = move_figure(
                board,
                black_figures,
                white_figures,
                figure,
                position.0,
                position.1,
                r#move.0 as u8,
                r#move.1 as u8,
                white,
            );

            if !is_board_valid(board.0, board.1, board.2, white) {
                r#move.0 = 127;
                r#move.1 = 127;
                subset[i] = r#move;
                break;
            }

            subset[i] = r#move;
        }

        moves[i] = subset
    }

    print_board_with_movements(board, moves, position.0, position.1);
}

fn handle_move_request(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
    input: (String, String),
) -> Result<(Board, Positions, Positions), String> {
    // Convert coordinates
    let mut chars = input.0.chars();
    let start_position: (u8, u8) = (
        chars.next().unwrap() as u8 - 'A' as u8 + 1,
        chars.next().unwrap() as u8 - '1' as u8 + 1,
    );

    let mut chars = input.1.chars();
    let end_position: (u8, u8) = (
        chars.next().unwrap() as u8 - 'A' as u8 + 1,
        chars.next().unwrap() as u8 - '1' as u8 + 1,
    );

    let figure = get_figure(board, start_position.0, start_position.1);

    if figure == 0 {
        return Err("There is no figure at the given position!".to_string());
    }

    if (white && figure / 10 != 0) || (!white && figure / 10 != 1) {
        return Err("The figure you want to move does not belong to you!".to_string());
    }

    let moves = get_valid_moves(board, figure, start_position.0, start_position.1);
    let resulting_board = move_figure(
        board,
        black_figures,
        white_figures,
        figure,
        start_position.0,
        start_position.1,
        end_position.0,
        end_position.1,
        white,
    );

    // Validate that the given move is valid
    if {
        // 1. Check if the given move is contained in the valid movement set.
        !contains_position(moves, end_position)
    } ||
        // 2. Check if the resulting board is valid (could the king be caught in the next turn?)
        !is_board_valid(resulting_board.0, resulting_board.1, resulting_board.2, white)
    {
        return Err(format!(
            "You cannot move {} at {} to {}!",
            get_str(figure).on_white(),
            input.0,
            input.1
        ));
    }

    println!(
        "Move {} at {} to {}",
        get_str(figure).on_white(),
        input.0,
        input.1
    );

    Ok(resulting_board)
}

pub fn is_checkmate(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
) -> bool {
    for figure in {
        if white {
            white_figures
        } else {
            black_figures
        }
    } {
        if figure.0 == 255 {
            continue;
        }
        let movement_set = get_valid_moves(board, figure.0, figure.1, figure.2);
        ////print!("{} at x{}y{}", get_str(figure.0), figure.1, figure.2);
        for subset in movement_set {
            for r#move in subset {
                if r#move.0 > 8 {
                    break;
                }

                ////print!(" to x{}y{} | ", r#move.0, r#move.1);

                let board = move_figure(
                    board,
                    black_figures,
                    white_figures,
                    figure.0,
                    figure.1,
                    figure.2,
                    r#move.0 as u8,
                    r#move.1 as u8,
                    white,
                );

                if is_board_valid(board.0, board.1, board.2, white) {
                    ////println!();
                    return false;
                }
            }
        }
        ////println!()
    }

    true
}

fn parse_input<'a>(input: String) -> Option<(String, String)> {
    let input: Vec<&str> = input.split_whitespace().collect();

    if input.len() > 2
        || input.len() < 1
        || !validate_coordinate(input[0])
        || (input.len() == 2 && (!validate_coordinate(input[1]) || input[0] == input[1]))
    {
        return None;
    }

    if input.len() == 2 {
        Some((input[0].chars().collect(), input[1].chars().collect()))
    } else {
        Some((input[0].chars().collect(), "".to_string()))
    }
}

fn validate_coordinate(coordinate: &str) -> bool {
    let coordinate: Vec<char> = coordinate.chars().collect();

    coordinate.len() == 2
        && (coordinate[0] >= 'A' && coordinate[0] <= 'H')
        && (coordinate[1] >= '1' && coordinate[1] <= '8')
}

#[allow(dead_code)]
pub fn print_moves(moves: MovementSet) {
    println!("[");
    for subset in moves {
        println!("    [");
        for position in subset {
            println!("    | {}-{}", position.0, position.1);
        }
        println!("    ]")
    }
    println!("]");
}

pub fn print_board(board: Board) {
    println!("    A B C D E F G H");
    println!("  ╔════════════════╗");
    for y in (1..=8).rev() {
        if y % 2 == 0 {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                get_str(get_figure(board, 1, y)).on_white(),
                get_str(get_figure(board, 2, y)).on_bright_black(),
                get_str(get_figure(board, 3, y)).on_white(),
                get_str(get_figure(board, 4, y)).on_bright_black(),
                get_str(get_figure(board, 5, y)).on_white(),
                get_str(get_figure(board, 6, y)).on_bright_black(),
                get_str(get_figure(board, 7, y)).on_white(),
                get_str(get_figure(board, 8, y)).on_bright_black(),
                PADDING.to_string().on_white(),
                PADDING.to_string().on_bright_black()
            );
        } else {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                get_str(get_figure(board, 1, y)).on_bright_black(),
                get_str(get_figure(board, 2, y)).on_white(),
                get_str(get_figure(board, 3, y)).on_bright_black(),
                get_str(get_figure(board, 4, y)).on_white(),
                get_str(get_figure(board, 5, y)).on_bright_black(),
                get_str(get_figure(board, 6, y)).on_white(),
                get_str(get_figure(board, 7, y)).on_bright_black(),
                get_str(get_figure(board, 8, y)).on_white(),
                PADDING.to_string().on_bright_black(),
                PADDING.to_string().on_white()
            );
        }
    }
    println!("  ╚════════════════╝");
    println!("    A B C D E F G H");
}

pub fn print_board_with_movements(
    board: Board,
    movements: MovementSet,
    active_x: u8,
    active_y: u8,
) {
    println!("    A B C D E F G H");
    println!("  ╔════════════════╗");
    for y in (1..=8).rev() {
        if y % 2 == 0 {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                {
                    let x = 1;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 2;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 3;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 4;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 5;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 6;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 7;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 8;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                PADDING.to_string().on_white(),
                PADDING.to_string().on_bright_black()
            );
        } else {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                {
                    let x = 1;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 2;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 3;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 4;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 5;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 6;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 7;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 8;
                    if x == active_x && y == active_y {
                        get_str(get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(get_figure(board, x, y)).on_white()
                    }
                },
                PADDING.to_string().on_bright_black(),
                PADDING.to_string().on_white()
            );
        }
    }
    println!("  ╚════════════════╝");
    println!("    A B C D E F G H");
}

fn is_in_movement_set(moves: MovementSet, x: u8, y: u8) -> bool {
    for subset in moves {
        for position in subset {
            if position.0 > 8 {
                break;
            }
            if position.0 as u8 == x && position.1 as u8 == y {
                return true;
            }
        }
    }

    return false;
}

/// Get a char representing either the figure at the given position or a space.
pub fn get_str(figure: Figure) -> ColoredString {
    match figure {
        WHITE_PAWN => CHAR_WHITE_PAWN.to_string().black().on_white(),
        WHITE_KING => CHAR_WHITE_KING.to_string().black().on_white(),
        WHITE_ROOK => CHAR_WHITE_ROOK.to_string().black().on_white(),
        WHITE_QUEEN => CHAR_WHITE_QUEEN.to_string().black().on_white(),
        WHITE_KNIGHT => CHAR_WHITE_KNIGHT.to_string().black().on_white(),
        WHITE_BISHOP => CHAR_WHITE_BISHOP.to_string().black().on_white(),
        BLACK_PAWN => CHAR_BLACK_PAWN.to_string().black().on_white(),
        BLACK_KING => CHAR_BLACK_KING.to_string().black().on_white(),
        BLACK_ROOK => CHAR_BLACK_ROOK.to_string().black().on_white(),
        BLACK_QUEEN => CHAR_BLACK_QUEEN.to_string().black().on_white(),
        BLACK_KNIGHT => CHAR_BLACK_KNIGHT.to_string().black().on_white(),
        BLACK_BISHOP => CHAR_BLACK_BISHOP.to_string().black().on_white(),
        _ => EMPTY_FIELD.to_string().black().on_white(),
    }
}
