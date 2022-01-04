use crate::engine::GameState;
mod engine;
mod io;

fn main() {
    let mut board = engine::INITIAL_BOARD;
    io::print_board(board);
    loop {
        board = match io::turn(board, true) {
            GameState::Normal(value) => value,
            GameState::CheckMate(value) => value,
            GameState::Check(value) => value,
        };
        board = match io::turn(board, false) {
            GameState::Normal(value) => value,
            GameState::CheckMate(value) => value,
            GameState::Check(value) => value,
        };
    }
}
