mod io;
mod engine;

fn main() {
    let mut board = [[' '; 8]; 8];
    loop {
        board = io::turn(board, true);
        board = io::turn(board, false);
    }
}