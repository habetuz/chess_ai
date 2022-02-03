extern crate core;
extern crate console_error_panic_hook;

mod engine;
mod ai;
mod io;

use wasm_timer;
use wasm_bindgen::prelude::*;
use crate::engine::{Board, GameState, get_figure, Positions};

static mut PREV_BOARDS: [Board; 10] = [engine::INITIAL_BOARD; 10];
static mut SEARCH_DEPTH: u8 = 2;
static mut FAST_CALC_COUNTER: u8 = 0;

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
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn turn(board: &str) -> String {
    unsafe {
        let board = parse_fen(board);

        let time = wasm_timer::Instant::now();

        let board = match ai::turn(board.0, board.1, board.2, false, SEARCH_DEPTH, PREV_BOARDS) {
            GameState::Normal(value) => {value}
            GameState::CheckMate(value) => {value}
        };

        for i in (1..PREV_BOARDS.len()).rev() {
            PREV_BOARDS[i] = PREV_BOARDS[i - 1];
        }

        PREV_BOARDS[0] = board.0;

        let time = time.elapsed().as_millis();
        if time < 2000 {
            FAST_CALC_COUNTER += 1;
            if FAST_CALC_COUNTER == 5 {
                SEARCH_DEPTH += 1;
                FAST_CALC_COUNTER = 0;
            }
        } else if time > 60000 {
            FAST_CALC_COUNTER = 0;
            if SEARCH_DEPTH > 2 {
                SEARCH_DEPTH -= 1;
            }
        } else {
            FAST_CALC_COUNTER = 0;
        }

        build_fen(board.0)
    }

}

fn parse_fen(fen: &str) -> (Board, Positions, Positions) {
    let mut fen = fen.chars();

    let mut board: Board = [0; 66];
    let mut black_figures: Positions = [(0,255,255); 16];
    let mut white_figures: Positions = [(0,255,255); 16];

    let mut board_index = 56;
    let mut x = 1;
    let mut y = 8;
    let mut board_parsed = false;

    while let Some(char) = fen.next() {
        if char == ' ' {
            if board_parsed {
                break;
            }
            board_parsed = true;
            fen.nth(1);
            continue;
        }

        if board_parsed {
            match char {
                'K' => board[64] += 2,
                'Q' => board[64] += 1,
                'k' => board[65] += 2,
                'q' => board[65] += 1,
                _ => panic!()
            }
            continue;
        }

        if char == '/' {
            y -= 1;
            x = 1;
            board_index -= 16;
            continue;
        }
        if char > '0' && char <= '8' {
            board_index += char as usize - '0' as usize;
            x += char as u8 - '0' as u8;
            continue;
        }

        match FEN_CHR.iter().position(|&x| x == char) {
            Some(figure) => {
                board[board_index] = figure as u8;
                if figure < 10 {
                    if white_figures[figure-1].0 == 0 {
                        white_figures[figure-1].0 = figure as u8;
                        white_figures[figure-1].1 = x;
                        white_figures[figure-1].2 = y;
                    } else if white_figures[figure].0 == 0 && figure != 2 {
                        white_figures[figure].0 = figure as u8;
                        white_figures[figure].1 = x;
                        white_figures[figure].2 = y;
                    } else {
                        for i in 8..16 {
                            if white_figures[i].0 == 0 {
                                white_figures[i].0 = figure as u8;
                                white_figures[i].1 = x;
                                white_figures[i].2 = y;
                                break;
                            }
                        }
                    }
                } else  {
                    let figure = figure - 10;
                    if black_figures[figure-1].0 == 0 {
                        black_figures[figure-1].0 = figure as u8 + 10;
                        black_figures[figure-1].1 = x;
                        black_figures[figure-1].2 = y;
                    } else if black_figures[figure].0 == 0 && figure != 12 {
                        black_figures[figure].0 = figure as u8 + 10;
                        black_figures[figure].1 = x;
                        black_figures[figure].2 = y;
                    } else {
                        for i in 8..16 {
                            if black_figures[i].0 == 0 {
                                black_figures[i].0 = figure as u8 + 10;
                                black_figures[i].1 = x;
                                black_figures[i].2 = y;
                                break;
                            }
                        }
                    }
                }
            },
            None => {}
        };

        board_index += 1;
        x += 1;
    }

    return (board, black_figures, white_figures)
}

fn build_fen(board: Board) -> String {
    let mut fen: String = String::new();
    let mut counter: u8 = 0;

    for y in (1..=8).rev() {
        for x in 1..=8 {
            if get_figure(board, x, y) == 0 {
                counter += 1;
            } else {
                if counter > 0 {
                    fen.push(char::from('0' as u8 + counter));
                    counter = 0;
                }
                fen.push(FEN_CHR[get_figure(board, x, y) as usize])

            }
        }

        if counter != 0 {
            fen.push(char::from('0' as u8 + counter));
            counter = 0;
        }

        if y != 1 {
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
