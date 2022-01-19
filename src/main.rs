use crate::engine::{Board, GameState};
use std::time::Instant;

mod ai;
mod engine;
mod io;

///
/// Main function. Wird beim Programmstart aufgerufen.
///
fn main() {
    // Initialisiert die `board` und `figures` Variablen, die die Informationen über das Spielbrett enthalten.
    let mut board = engine::INITIAL_BOARD;
    let mut black_figures = engine::POSITIONS_BLACK;
    let mut white_figures = engine::POSITIONS_WHITE;

    let mut prev_boards = [board; 10];

    let mut fast_calc_counter = 0;
    let mut search_depth = 3;

    println!(
        "Start evaluation: {}",
        ai::evaluate_board(black_figures, white_figures)
    );

    // Der Rückgabewert des loops wird in `end_board` gespeichert. Siehe [`quick_rust_explanation: Rückgabe von Werten`] wie die Rückgabe von Werten in Rust funktioniert.
    let end_board: (Board, bool) = loop {
        io::print_board(board);

        // `io::turn` ist die Zug-Funktion, wenn ein Mensch ziehen soll.
        match io::turn(board, black_figures, white_figures, true) {
            GameState::Normal(value) => {
                board = value.0;
                black_figures = value.1;
                white_figures = value.2;
            }
            GameState::CheckMate(value) => {
                break (value.0, true);
            }
        };

        io::print_board(board);

        let time = Instant::now();

        // `ai::turn` ist die Zug-Funktion, wenn die KI ziehen zoll.
        match ai::turn(
            board,
            black_figures,
            white_figures,
            false,
            search_depth,
            prev_boards,
        ) {
            GameState::Normal(value) => {
                board = value.0;
                black_figures = value.1;
                white_figures = value.2;

                for i in (1..prev_boards.len()).rev() {
                    prev_boards[i] = prev_boards[i - 1];
                }

                prev_boards[0] = board;
            }
            GameState::CheckMate(value) => {
                break (value.0, false);
            }
        };

        let time = time.elapsed().as_millis();
        if time < 2000 {
            fast_calc_counter += 1;
            if fast_calc_counter == 5 {
                search_depth += 1;
                println!("AI> Increasing search depth to {}!", search_depth);
                fast_calc_counter = 0;
            }
        } else if time > 120000 {
            fast_calc_counter = 0;
            if search_depth > 3 {
                search_depth -= 1;
                println!("AI> Decreasing search depth to {}!", search_depth);
            }
        } else {
            fast_calc_counter = 0;
        }
    };

    // Das Sieger-Spielbrett wird ausgegeben.
    io::print_board(end_board.0);

    // Und eine Nachricht mit dem Gewinner wird ausgegeben.
    println!("WOW! {} hat gewonnen!", {
        if end_board.1 {
            "Weiß"
        } else {
            "Schwarz"
        }
    });
}
