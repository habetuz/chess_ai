mod evaluation;

use crate::ai::evaluation::{EG_TABLES, EG_VALUES, GAME_PHASE_INC, MG_TABLES, MG_VALUES};
use crate::engine::figures::{BLACK_KING, WHITE_KING};
use crate::engine::{get_valid_moves, is_board_valid, move_figure, Board, Positions};
use crate::io::get_str;
use crate::io::is_checkmate;
use crate::GameState;
use crate::GameState::{CheckMate, Normal};
use std::io::Write;

pub fn turn(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
    turns: u8,
) -> GameState {
    let depth = turns * 2;

    let mut best_value = if white { i32::MIN } else { i32::MAX };
    let mut best_board = (board, black_figures, white_figures);
    let mut best_move = ((1u8, 1u8, 1u8), (1i8, 1i8));

    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    if white {
        println!("       KDRRLLTTBBBBBBBB -> Fortschritt Bedeutung");
    } else {
        println!("          KDRRLLTTBBBBBBBB -> Fortschritt Bedeutung");
    }

    let mut progress = 0;
    print_progress(progress, white);

    'outer: for figure in {
        if white {
            white_figures
        } else {
            black_figures
        }
    } {
        if figure.0 == 255 {
            progress += 1;
            continue;
        }

        let moves = get_valid_moves(board, figure.0, figure.1, figure.2);

        for subset in moves {
            for r#move in subset {
                if r#move.0 > 8 {
                    break;
                }

                let (board, black_figures, white_figures) = move_figure(
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

                let value = min_max(
                    board,
                    black_figures,
                    white_figures,
                    !white,
                    alpha,
                    beta,
                    depth - 1,
                );

                if (white && value > best_value) || (!white && value < best_value) {
                    best_value = value;
                    best_board = (board, black_figures, white_figures);
                    best_move = (figure, (r#move.0, r#move.1));
                }

                if white && value > alpha {
                    alpha = value;
                } else if !white && value < beta {
                    beta = value;
                }

                if beta <= alpha {
                    print_progress(16, white);
                    break 'outer;
                }
            }
        }

        progress += 1;
        print_progress(progress, white);
    }
    println!(
        " Mindestens erreichbarer Wert: {}. Bewegt {} an {}{} nach {}{}",
        best_value,
        get_str(best_move.0 .0),
        ('A' as u8 + best_move.0 .1 - 1) as char,
        best_move.0 .2,
        ('A' as u8 + best_move.1 .0 as u8 - 1) as char,
        best_move.1 .1
    );

    if is_checkmate(best_board.0, best_board.1, best_board.2, !white) {
        CheckMate(best_board)
    } else {
        Normal(best_board)
    }
}

fn print_progress(progress: u8, white: bool) {
    if white {
        print!("\rWeiß> [");
    } else {
        print!("\rSchwarz> [");
    }

    for i in 0..16 {
        if i < progress {
            print!("#");
        } else {
            print!(" ");
        }
    }

    print!("]");

    std::io::stdout().flush().expect("Could not flush stdout!");
}

pub fn min_max(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
    mut alpha: i32,
    mut beta: i32,
    depth: u8,
) -> i32 {
    if depth == 0 {
        if !is_board_valid(board, black_figures, white_figures, !white) {
            return if white { i32::MAX } else { i32::MIN };
        }
        return evaluate_board(black_figures, white_figures);
    }

    if black_figures[0].0 == 255 {
        return i32::MAX;
    } else if white_figures[0].0 == 255 {
        return i32::MIN;
    }

    let mut min = i32::MAX;
    let mut max = i32::MIN;

    'outer: for figure in {
        if white {
            white_figures
        } else {
            black_figures
        }
    } {
        if figure.0 == 255 {
            continue;
        }

        let moves = get_valid_moves(board, figure.0, figure.1, figure.2);

        for subset in moves {
            for r#move in subset {
                if r#move.0 > 8 {
                    break;
                }

                let (board, black_figures, white_figures) = move_figure(
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

                let var = min_max(
                    board,
                    black_figures,
                    white_figures,
                    !white,
                    alpha,
                    beta,
                    depth - 1,
                );

                if white && var == i32::MAX {
                    return i32::MAX;
                } else if !white && var == i32::MIN {
                    return i32::MIN;
                }

                if var < min {
                    if !white {
                        beta = var;
                    }
                    min = var;
                }
                if var > max {
                    if white {
                        alpha = var;
                    }
                    max = var;
                }

                if beta <= alpha {
                    break 'outer;
                }
            }
        }
    }

    return if white { max } else { min };
}

// Evaluation function inspired by PeSTO: https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function
fn evaluate_board(black_figures: Positions, white_figures: Positions) -> i32 {
    let mut mg_score = 0;
    let mut eg_score = 0;

    let mut game_phase = 0;

    for i in 0..black_figures.len() {
        let white = white_figures[i];
        let black = black_figures[i];

        if white.0 != 255 {
            assert!(white.0 == WHITE_KING || EG_VALUES[white.0 as usize] != 0);

            mg_score += MG_VALUES[white.0 as usize]
                + MG_TABLES[white.0 as usize][(8 * (white.2 - 1) + (white.1 - 1)) as usize];
            eg_score += EG_VALUES[white.0 as usize]
                + EG_TABLES[white.0 as usize][(8 * (white.2 - 1) + (white.1 - 1)) as usize];

            game_phase += GAME_PHASE_INC[white.0 as usize]
        }

        if black.0 != 255 {
            assert!(black.0 == BLACK_KING || EG_VALUES[(black.0 - 10) as usize] != 0);

            mg_score -= MG_VALUES[(black.0 - 10) as usize]
                + MG_TABLES[(black.0 - 10) as usize]
                    [((8 * (7 - (black.2 - 1)) + (7 - (black.1 - 1))) as usize)];
            eg_score -= EG_VALUES[(black.0 - 10) as usize]
                + EG_TABLES[(black.0 - 10) as usize]
                    [((8 * (7 - (black.2 - 1)) + (7 - (black.1 - 1))) as usize)];

            game_phase += GAME_PHASE_INC[(black.0 - 10) as usize]
        }
    }

    let mg_phase = if game_phase > 24 { 24 } else { game_phase };
    let eg_phase = 24 - mg_phase;
    (mg_score * mg_phase + eg_score * eg_phase) / 24
}
