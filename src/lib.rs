extern crate core;

mod engine;
mod ai;
mod io;

use wasm_bindgen::prelude::*;
use crate::engine::{Board, GameState, Positions};

static mut PREV_BOARDS: [Board; 10] = [engine::INITIAL_BOARD; 10];

const FEN_CHR: [char; 20] = [
    ' ',
    'K',
    'Q',
    'N',
    ' ',
    'B',
    ' ',
    'R',
    ' ',
    'P',
    ' ',
    'k',
    'q',
    'n',
    ' ',
    'b',
    ' ',
    'r',
    ' ',
    'p',
];

#[cfg(test)]
mod tests {
    use crate::{build_fen, engine, parse_fen};

    #[test]
    fn fen_parsing() {
        let board = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(board.0, engine::INITIAL_BOARD);

        assert_eq!(build_fen(board.0), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn turn(board: &str) -> String {
    let board = parse_fen(board);
    build_fen(board.0)
}

fn parse_fen(fen: &str) -> (Board, Positions, Positions) {
    let fen = fen.chars();

    let mut board: Board = [0; 66];
    let mut black_figures: Positions = [(0,255,255); 16];
    let mut white_figures: Positions = [(0,255,255); 16];

    let mut board_index = 63;
    let mut x = 1;
    let mut y = 8;

    for char in fen {
        if char == '/' {
            y -= 1;
            x = 1;
            continue;
        }
        if '0' as u8 + char as u8 - 1 <= 8 {
            board_index += '0' as usize + char as usize - 1;
            x += '0' as u8 + char as u8 - 1;
            continue;
        }

        match FEN_CHR.iter().position(|&x| x == char) {
            Some(figure) => {
                board[board_index] = figure as u8;
                if figure < 10 {
                    if white_figures[figure].0 == 0 {
                        white_figures[figure].0 = figure as u8;
                        white_figures[figure].1 = x;
                        white_figures[figure].2 = y;
                    } else if white_figures[figure + 1].0 == 0 && figure != 2 {
                        white_figures[figure].0 = figure as u8;
                        white_figures[figure].1 = x;
                        white_figures[figure].2 = y;
                    } else {
                        for i in 9..16 {
                            if white_figures[i].0 == 0 {
                                white_figures[i].0 = figure as u8;
                                white_figures[i].1 = x;
                                white_figures[i].2 = y;
                            }
                        }
                    }
                } else  {
                    let figure = figure - 10;
                    if black_figures[figure].0 == 0 {
                        black_figures[figure].0 = figure as u8;
                        black_figures[figure].1 = x;
                        black_figures[figure].2 = y;
                    } else if black_figures[figure + 1].0 == 0 && figure != 12 {
                        black_figures[figure].0 = figure as u8;
                        black_figures[figure].1 = x;
                        black_figures[figure].2 = y;
                    } else {
                        for i in 9..16 {
                            if black_figures[i].0 == 0 {
                                black_figures[i].0 = figure as u8;
                                black_figures[i].1 = x;
                                black_figures[i].2 = y;
                            }
                        }
                    }
                }
            },
            None => {}
        }

        board_index -= 1;
        x += 1;
    }

    return (board, black_figures, white_figures)
}

fn build_fen(board: Board) -> String {
    let mut fen: String = String::new();
    let mut counter: u8 = 0;
    for i in 0..64 {
        let chr = FEN_CHR[board[i] as usize];
        if chr == ' ' {
            counter += 1;
        } else {
            if counter != 0 {
                fen.push(char::from(counter));
                counter = 0;
            }
            fen.push(FEN_CHR[i]);
        }

        if i % 7 == 0 {
            if counter != 0 {
                fen.push(char::from(counter));
                counter = 0;
            }
            fen.push('/');
        }
    }

    fen.push_str(" w ");

    let castling = {
        let mut castling = String::new();
        match board[64] {
            1 => castling.push('Q'),
            2 => castling.push('K'),
            3 => castling.push_str("KQ"),
            0 => {},
            _ => panic!()
        }

        match board[65] {
            1 => castling.push('q'),
            2 => castling.push('K'),
            3 => castling.push_str("kq"),
            0 => {},
            _ => panic!()
        }
        castling
    };

    if castling == "" {
        fen.push(' ');
    } else {
        fen.push_str(&*castling);
    }

    fen.push_str(" - 0 1");

    fen
}
