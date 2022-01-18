use crate::engine::{Board, GameState};

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

        // `ai::turn` ist die Zug-Funktion, wenn die KI ziehen zoll.
        // TODO: Die `turn` anzahl soll sich erhöhen, wenn die KI mehrere Züge kürzer als eine bestimmte Zeit benötigt.
        match ai::turn(board, black_figures, white_figures, false, 3) {
            GameState::Normal(value) => {
                board = value.0;
                black_figures = value.1;
                white_figures = value.2;
            }
            GameState::CheckMate(value) => {
                break (value.0, true);
            }
        };
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
