use crate::engine;
use crate::engine::figures;
use crate::GameState::{CheckMate, Normal};
use colored;
use colored::Colorize;
use std::io::Write;

/// Zug-Funktion für Menschen.
///
/// # Parameter
/// * `board`: [`engine::Board`] | Das Aktuelle Spielbrett.
/// * `black_figures`: [`engine::Positions`] | Die Positionen der schwarzen Figuren.
/// * `white_figures`: [`engine::Positions`] | Die Positionen der weißen Figuren.
/// * `white`: `true`, wenn der ziehende Spieler weiß ist.
///
/// # Rückgabewert `engine::GameState`
/// Entweder einen [`engine::GameState::Normal`] zurück, das das neue Spielbrett enthält, oder einen [`engine::GameState::CheckMate`] wenn der ziehende Spieler gewonnen hat.
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
    if engine::is_checkmate(
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

/// Funktion, die alle möglichen Züge für eine Figur findet und in die Konsole ausgibt.
///
/// # Parameter
/// * `board`: [`engine::Board`] | Aktuelles Spielbrett.
/// * `black_figures`: [`engine::Positions`] | Positionen der schwarzen Figuren.
/// * `white_figures`: [`engine::Positions`] | Positionen der weißen Figuren.
/// * `white` : `bool` | `true`, wenn weiß am Zug ist.
/// * `input` : `String` | Validierter input in `String`-Form.
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

            // Siehe engine::figures::MovementSet für eine Erklärung,
            // warum die Suche in den MovementSubSets beendet werden kann, sobald ein Zug eine größere Koordinate als 8 besitzt.
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

            // Falls das Board nicht valide ist (falls der König im nächsten Zug geschmissen werden könnte) wird die weiter Suche in diesem MovementSubSet beendet.
            // Again: Siehe engine::figures::MovementSet für eine Erklärung.
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

    // Am ende werden die validen moves an die print methode weitergegeben.
    print_board_with_movements(board, moves, position.0, position.1);
}

/// Funktion, die versucht einen vom Spieler gewünschten Zug auszuführen.
///
/// # Parameter
/// * `board`: [`engine::Board`] | Aktuelles Spielbrett.
/// * `black_figures`: [`engine::Positions`] | Positionen der schwarzen Figuren.
/// * `white_figures`: [`engine::Positions`] | Positionen der weißen Figuren.
/// * `white` : `bool` | `true`, wenn weiß am Zug ist.
/// * `input` : `(String, String)` | Validierter input in (von (als `String`), nach (als `String`)-Form.
///
/// # Rückgabewert `Result<(engine::Board, engine::Positions, engine::Positions), String>`
/// → Siehe [`Result`], [`engine::Board`], [`engine::Positions`]
///
/// Gibt [`Result::Ok`] mit dem Spielbrett nach dem Zug oder [`Result::Err`] mit einer Fehlernachricht zurück.
fn handle_move_request(
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
    white: bool,
    input: (String, String),
) -> Result<(engine::Board, engine::Positions, engine::Positions), String> {
    // Konvertierung der String-Darstellung zu der `(u8, u8)`-Darstellung der Koordinaten.
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

    // Die Figur, die bewegt werden soll.
    let figure = engine::get_figure(board, start_position.0, start_position.1);

    // Falls die Figur 0 ist, wird ein Error zurückgegeben.
    if figure == 0 {
        return Err("An diese Position ist keine Figur!".to_string());
    }

    // Überprüft, ob die Figur auch dem Spieler gehört (und gibt einen Error zurück, falls dies nicht der Fall ist.
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

    // Validiert den Zug und gibt gegebenenfalls einen Error aus.
    if {
        // 1. Überprüft, dass die Position, an die der Spieler ziehen möchte, auch im MovementSet enthalten ist.
        !engine::contains_position(moves, end_position)
    } ||
        // 2. Überprüft ob das Spielbrett valide ist, indem überprüft wird, ob der König im darauffolgenden Zug geschlagen werden könnte.
        !engine::is_board_valid(resulting_board.0, resulting_board.1, resulting_board.2, white)
    {
        return Err(format!(
            "Figur {} auf {} kann nicht nach {} bewegt werden!",
            get_str(figure).on_white(),
            input.0,
            input.1
        ));
    }

    println!(
        "Bewege {} von {} nach {}",
        get_str(figure).on_white(),
        input.0,
        input.1
    );

    Ok(resulting_board)
}

/// Parst den vom Spieler gegebenen Input mit der Gewünschten Aktion, falls möglich.
///
/// # Parameter
/// * `input`: Input vom Spieler
///
/// # Rückgabewert `Option<(String, String)>`
/// → Siehe [`Option`]
///
/// Gibt den Input auf gespaltet in zwei Teile wieder.
fn parse_input<'a>(input: String) -> Option<(String, String)> {
    let input: Vec<&str> = input.split_whitespace().collect();

    // Validiert den Input
    if input.len() > 2
        || input.len() < 1
        || !validate_coordinate(input[0])
        || (input.len() == 2 && (!validate_coordinate(input[1]) || input[0] == input[1]))
    {
        return None;
    }

    // Gibt je nach der Länge nur eine oder zwei Koordinaten zurück.
    if input.len() == 2 {
        Some((input[0].chars().collect(), input[1].chars().collect()))
    } else {
        Some((input[0].chars().collect(), "".to_string()))
    }
}

/// Validiert eine Koordinate.
///
/// # Parameter
/// * `coordinate`: Die Koordinate, die validiert werden soll.
///
/// # Rückgabewert `bool`
/// Gibt `true` zurück, falls die Koordinate valide ist.
fn validate_coordinate(coordinate: &str) -> bool {
    let coordinate: Vec<char> = coordinate.chars().collect();

    coordinate.len() == 2
        && (coordinate[0] >= 'A' && coordinate[0] <= 'H')
        && (coordinate[1] >= '1' && coordinate[1] <= '8')
}

/// Gibt ein Spielbrett aus.
///
/// # Parameter
/// * `board`: Das [`engine::Board`], das ausgegeben werden soll.
pub fn print_board(board: engine::Board) {
    println!("    A B C D E F G H");
    println!("  ╔════════════════╗");
    // Iteriere durch die Zeilen (von 8 -> 1)
    for y in (1..=8).rev() {
        // Wenn y eine gerade Reihe ist: Zuerst ein weißes Feld.
        if y % 2 == 0 {
            println!(
                "{0} ║{1}{2}{3}{4}{5}{6}{7}{8}║ {0}",
                y,
                get_str(engine::get_figure(board, 1, y)).on_white(),
                get_str(engine::get_figure(board, 2, y)).on_bright_black(),
                get_str(engine::get_figure(board, 3, y)).on_white(),
                get_str(engine::get_figure(board, 4, y)).on_bright_black(),
                get_str(engine::get_figure(board, 5, y)).on_white(),
                get_str(engine::get_figure(board, 6, y)).on_bright_black(),
                get_str(engine::get_figure(board, 7, y)).on_white(),
                get_str(engine::get_figure(board, 8, y)).on_bright_black(),
            );
        } else {
            println!(
                "{0} ║{1}{2}{3}{4}{5}{6}{7}{8}║ {0}",
                y,
                get_str(engine::get_figure(board, 1, y)).on_bright_black(),
                get_str(engine::get_figure(board, 2, y)).on_white(),
                get_str(engine::get_figure(board, 3, y)).on_bright_black(),
                get_str(engine::get_figure(board, 4, y)).on_white(),
                get_str(engine::get_figure(board, 5, y)).on_bright_black(),
                get_str(engine::get_figure(board, 6, y)).on_white(),
                get_str(engine::get_figure(board, 7, y)).on_bright_black(),
                get_str(engine::get_figure(board, 8, y)).on_white(),
            );
        }
    }
    println!("  ╚════════════════╝");
    println!("    A B C D E F G H");
}

/// Gibt ein Spielbrett aus und markiert die gegeben Zugmöglichkeiten und die ziehende Figur.
///
/// # Parameter
/// * `board`: Das [`engine::Board`], das ausgegeben werden soll.
/// * `movements`: Das [`figures::MovementSet`], das markiert werden soll.
/// * `aktive_x`: Die x-Koordinate der aktiven Figur.
/// * `aktive_y`: Die y-Koordinate der aktiven Figur.
pub fn print_board_with_movements(
    board: engine::Board,
    movements: figures::MovementSet,
    active_x: u8,
    active_y: u8,
) {
    println!("    A B C D E F G H");
    println!("  ╔════════════════╗");
    // Iteriere durch die Zeilen (von 8 -> 1)
    for y in (1..=8).rev() {
        // Wenn y eine gerade Reihe ist: Zuerst ein weißes Feld.
        if y % 2 == 0 {
            println!(
                "{0} ║{1}{2}{3}{4}{5}{6}{7}{8}║ {0}",
                y,
                {
                    let x = 1;
                    // Wenn es sich um die aktive Figur handelt, wird der Hintergrund rot gefärbt.
                    if x == active_x && y == active_y {
                        get_str(engine::get_figure(board, x, y)).on_red()
                    // Wenn das Feld ein möglicher Zug ist, wird der Hintergrund gelb gefärbt.
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
            );
        } else {
            println!(
                "{0} ║{1}{2}{3}{4}{5}{6}{7}{8}║ {0}",
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
            );
        }
    }
    println!("  ╚════════════════╝");
    println!("    A B C D E F G H");
}

/// Überprüft, ob eine Koordinate in einem MovementSet enthalten ist
///
/// # Parameter
/// * `moves`: Das [`figures::MovementSet`] in dem der Zug enthalten sein soll.
/// * `x`: Die x-Koordinate des Zuges.
/// * `y`: Die y-Koordinate des Zuges.
///
/// # Rückgabewert `bool`
/// `true` falls der Zug im [`figures::MovementSet`] enthalten ist.
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

/// Konvertiert die gegebene [`figures::Figure`] zu einem [`colored::ColoredString`].
///
/// # Parameter
/// * `figure`: Die [`figures::Figure`], die konvertiert werden soll.
///
/// # Rückgabewert `colored::ColoredString`
/// Gibt die Figur als [`colored::ColoredString`] mit schwarzem Vordergrund und weißem Hintergrund zurück.
pub fn get_str(figure: figures::Figure) -> colored::ColoredString {
    let r#char = match figure {
        figures::WHITE_PAWN => figures::CHAR_WHITE_PAWN,
        figures::WHITE_KING => figures::CHAR_WHITE_KING,
        figures::WHITE_ROOK => figures::CHAR_WHITE_ROOK,
        figures::WHITE_QUEEN => figures::CHAR_WHITE_QUEEN,
        figures::WHITE_KNIGHT => figures::CHAR_WHITE_KNIGHT,
        figures::WHITE_BISHOP => figures::CHAR_WHITE_BISHOP,
        figures::BLACK_PAWN => figures::CHAR_BLACK_PAWN,
        figures::BLACK_KING => figures::CHAR_BLACK_KING,
        figures::BLACK_ROOK => figures::CHAR_BLACK_ROOK,
        figures::BLACK_QUEEN => figures::CHAR_BLACK_QUEEN,
        figures::BLACK_KNIGHT => figures::CHAR_BLACK_KNIGHT,
        figures::BLACK_BISHOP => figures::CHAR_BLACK_BISHOP,
        _ => ' ',
    };

    format!(" {}", r#char).black().on_white()
}
