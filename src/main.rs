use crate::engine::GameState;
use crate::io::is_checkmate;

mod engine;
mod io;

fn main() {
    let mut board = engine::INITIAL_BOARD;
    let mut black_figures = engine::POSITIONS_BLACK;
    let mut white_figures = engine::POSITIONS_WHITE;
    assert!(!is_checkmate(board, black_figures, white_figures, true));
    let end_board = loop {
        io::print_board(board);

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

        match io::turn(board, black_figures, white_figures, false) {
            GameState::Normal(value) => {
                board = value.0;
                black_figures = value.1;
                white_figures = value.2;
            }
            GameState::CheckMate(value) => {
                break (value.0, false);
            }
        };
    };

    io::print_board(end_board.0);
    println!("WOW! {} won the game!", {
        if end_board.1 {
            "White"
        } else {
            "Black"
        }
    })
}
