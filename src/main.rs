use crate::engine::GameState;
mod engine;
mod io;

fn main() {
    let mut board = [[' '; 8]; 8];
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
