use crate::engine;
use crate::engine::figures;
use crate::GameState::{CheckMate, Normal};
use colored;
use colored::Colorize;
use std::io::Write;

const PADDING: char = ' ';
const EMPTY_FIELD: char = ' ';

/// # Funktion `io::turn`
///
/// Zug-Funktion für Menschen.
///
/// ---
/// ## Parameter
/// `board`: [`engine::Board`] | Das Aktuelle Spielbrett.
///
/// `black_figures`: [`engine::Positions`] | Die Positionen der schwarzen Figuren.
///
/// `black_figures`: [`engine::Positions`] | Die Positionen der weißen Figuren.
///
/// ---
/// ## Rückgabewert [`engine::GameState`]
/// Gibt entweder einen [`engine::GameState::Normal`] zurück, das das neue Spielbrett enthält, oder einen [`engine::GameState::CheckMate`] wenn der ziehende Spieler gewonnen hat.
pub fn turn(
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
    white: bool,
) -> engine::GameState {
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

/// # Funktion `io::handle_possible_moves_request`
/// Funktion, die alle möglichen Züge für eine Figur findet und in die Konsole ausgibt.
///
/// ---
/// ## Parameter
/// `board`: [`engine::Board`] | Aktuelles Spielbrett.
///
/// `black_figures`: [`engine::Positions`] | Positionen der schwarzen Figuren.
///
/// `white_figures`: [`engine::Positions`] | Positionen der weißen Figuren.
///
/// `white` : `bool` | `true`, wenn weiß am Zug ist.
///
/// `input` : `String` | Validierter input in `String` Form.

fn handle_possible_moves_request(
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
    white: bool,
    input: String,
) {
    // Konvertierung der `String`-Darstellung zu der `(u8, u8)`-Darstellung der Koordinate.
    let mut chars = input.chars();
    let position: (u8, u8) = (
        chars.next().unwrap() as u8 - 'A' as u8 + 1,
        chars.next().unwrap() as u8 - '1' as u8 + 1,
    );

    // Ausfiltern, falls an der gegebenen Position keine Figur ist, oder diese Figur dem ziehenden Spieler nicht gehört.
    let figure = engine::get_figure(board, position.0, position.1);
    if figure == 0 {
        println!("An diese Position ist keine Figur!");
        return;
    }

    if (white && figure / 10 != 0) || (!white && figure / 10 != 1) {
        println!("Diese Figur gehört nicht dir!");
        return;
    }

    // Durch alle validen Züge iterieren.
    let mut moves = engine::get_valid_moves(board, figure, position.0, position.1);
    for i in 0..moves.len() {
        let mut subset = moves[i];
        for i in 0..subset.len() {
            let mut r#move = subset[i];

            // Siehe `engine::figures::MovementSet` für eine Erklärung,
            // warum die Suche in den `MovementSubSet`s beendet werden kann, sobald ein Zug eine größere Koordinate als 8 besitzt.
            if r#move.0 > 8 {
                break;
            }

            let board = engine::move_figure(
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

            // Falls das Board nicht valide ist (falls der König im nächsten Zug geschmissen werden könnte) wird die weiter Suche in diesem `MovementSubSet` beendet.
            // Again: Siehe `engine::figures::MovementSet` für eine Erklärung.
            if !engine::is_board_valid(board.0, board.1, board.2, white) {
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

/// # Funktion `io::handle_move_request`
fn handle_move_request(
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
    white: bool,
    input: (String, String),
) -> Result<(engine::Board, engine::Positions, engine::Positions), String> {
    // Konvertierung der `String`-Darstellung zu der `(u8, u8)`-Darstellung der Koordinaten.
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

    let figure = engine::get_figure(board, start_position.0, start_position.1);

    if figure == 0 {
        return Err("An diese Position ist keine Figur!".to_string());
    }

    if (white && figure / 10 != 0) || (!white && figure / 10 != 1) {
        return Err("Diese Figur gehört nicht dir!".to_string());
    }

    let moves = engine::get_valid_moves(board, figure, start_position.0, start_position.1);
    let resulting_board = engine::move_figure(
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
        !engine::contains_position(moves, end_position)
    } ||
        // 2. Check if the resulting board is valid (could the king be caught in the next turn?)
        !engine::is_board_valid(resulting_board.0, resulting_board.1, resulting_board.2, white)
    {
        return Err(format!(
            "Figur {} an {} kann nicht nach {} bewegt werden!",
            get_str(figure).on_white(),
            input.0,
            input.1
        ));
    }

    println!(
        "Bewege {} an {} nach {}",
        get_str(figure).on_white(),
        input.0,
        input.1
    );

    Ok(resulting_board)
}

pub fn is_checkmate(
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
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
        let movement_set = engine::get_valid_moves(board, figure.0, figure.1, figure.2);
        ////print!("{} at x{}y{}", get_str(figure.0), figure.1, figure.2);
        for subset in movement_set {
            for r#move in subset {
                if r#move.0 > 8 {
                    break;
                }

                ////print!(" to x{}y{} | ", r#move.0, r#move.1);

                let board = engine::move_figure(
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

                if engine::is_board_valid(board.0, board.1, board.2, white) {
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
pub fn print_moves(moves: figures::MovementSet) {
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

pub fn print_board(board: engine::Board) {
    println!("    A B C D E F G H");
    println!("  ╔════════════════╗");
    for y in (1..=8).rev() {
        if y % 2 == 0 {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                get_str(engine::get_figure(board, 1, y)).on_white(),
                get_str(engine::get_figure(board, 2, y)).on_bright_black(),
                get_str(engine::get_figure(board, 3, y)).on_white(),
                get_str(engine::get_figure(board, 4, y)).on_bright_black(),
                get_str(engine::get_figure(board, 5, y)).on_white(),
                get_str(engine::get_figure(board, 6, y)).on_bright_black(),
                get_str(engine::get_figure(board, 7, y)).on_white(),
                get_str(engine::get_figure(board, 8, y)).on_bright_black(),
                PADDING.to_string().on_white(),
                PADDING.to_string().on_bright_black()
            );
        } else {
            println!(
                "{0} ║{9}{1}{10}{2}{9}{3}{10}{4}{9}{5}{10}{6}{9}{7}{10}{8}║ {0}",
                y,
                get_str(engine::get_figure(board, 1, y)).on_bright_black(),
                get_str(engine::get_figure(board, 2, y)).on_white(),
                get_str(engine::get_figure(board, 3, y)).on_bright_black(),
                get_str(engine::get_figure(board, 4, y)).on_white(),
                get_str(engine::get_figure(board, 5, y)).on_bright_black(),
                get_str(engine::get_figure(board, 6, y)).on_white(),
                get_str(engine::get_figure(board, 7, y)).on_bright_black(),
                get_str(engine::get_figure(board, 8, y)).on_white(),
                PADDING.to_string().on_bright_black(),
                PADDING.to_string().on_white()
            );
        }
    }
    println!("  ╚════════════════╝");
    println!("    A B C D E F G H");
}

pub fn print_board_with_movements(
    board: engine::Board,
    movements: figures::MovementSet,
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
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 2;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 3;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 4;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 5;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 6;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 7;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 8;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
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
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 2;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 3;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 4;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 5;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 6;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
                    }
                },
                {
                    let x = 7;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_bright_black()
                    }
                },
                {
                    let x = 8;
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    } else if is_in_movement_set(movements, x, y) {
                        get_str(engine::get_figure(board, x, y)).on_yellow()
                    } else {
                        get_str(engine::get_figure(board, x, y)).on_white()
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

fn is_in_movement_set(moves: figures::MovementSet, x: u8, y: u8) -> bool {
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
pub fn get_str(figure: figures::Figure) -> colored::ColoredString {
    match figure {
        figures::WHITE_PAWN => figures::CHAR_WHITE_PAWN.to_string().black().on_white(),
        figures::WHITE_KING => figures::CHAR_WHITE_KING.to_string().black().on_white(),
        figures::WHITE_ROOK => figures::CHAR_WHITE_ROOK.to_string().black().on_white(),
        figures::WHITE_QUEEN => figures::CHAR_WHITE_QUEEN.to_string().black().on_white(),
        figures::WHITE_KNIGHT => figures::CHAR_WHITE_KNIGHT.to_string().black().on_white(),
        figures::WHITE_BISHOP => figures::CHAR_WHITE_BISHOP.to_string().black().on_white(),
        figures::BLACK_PAWN => figures::CHAR_BLACK_PAWN.to_string().black().on_white(),
        figures::BLACK_KING => figures::CHAR_BLACK_KING.to_string().black().on_white(),
        figures::BLACK_ROOK => figures::CHAR_BLACK_ROOK.to_string().black().on_white(),
        figures::BLACK_QUEEN => figures::CHAR_BLACK_QUEEN.to_string().black().on_white(),
        figures::BLACK_KNIGHT => figures::CHAR_BLACK_KNIGHT.to_string().black().on_white(),
        figures::BLACK_BISHOP => figures::CHAR_BLACK_BISHOP.to_string().black().on_white(),
        _ => EMPTY_FIELD.to_string().black().on_white(),
    }
}
