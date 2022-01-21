mod evaluation;

use crate::engine;
use crate::io;
use std::io::Write;

pub fn turn(
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
    white: bool,
    turns: u8,
    prev_boards: [engine::Board; 10],
) -> engine::GameState {
    let depth = turns * 2;

    let mut best_value = if white { i32::MIN } else { i32::MAX };
    let mut best_board = (board, black_figures, white_figures);
    let mut best_move = ((255u8, 1u8, 1u8), (1i8, 1i8));

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

        let moves = engine::get_valid_moves(board, figure.0, figure.1, figure.2);

        for subset in moves {
            for r#move in subset {
                if r#move.0 > 8 {
                    break;
                }

                let (board, black_figures, white_figures) = engine::move_figure(
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

                let different = {
                    let mut different = [false; 10];

                    for i in 0..64 {
                        for j in 0..different.len() {
                            if board[i] != prev_boards[j][i] {
                                different[j] = true;
                            }
                        }
                        if different.iter().all(|&x| x) {
                            break;
                        }
                    }

                    different.iter().all(|&x| x)
                };

                if !different {
                    println!(" AI> Detected a repetition move!");
                    continue;
                }

                let value = min_max(
                    board,
                    black_figures,
                    white_figures,
                    !white,
                    alpha,
                    beta,
                    depth - 1,
                );

                if white && value > alpha {
                    alpha = value;
                    best_value = value;
                    best_board = (board, black_figures, white_figures);
                    best_move = (figure, (r#move.0, r#move.1));
                } else if !white && value < beta {
                    beta = value;
                    best_value = value;
                    best_board = (board, black_figures, white_figures);
                    best_move = (figure, (r#move.0, r#move.1));
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
        "\nMindestens erreichbarer Wert: {}. \nAktueller Wert: {}\nBewegt {} von {}{} nach {}{}",
        best_value,
        evaluate_board(best_board.1, best_board.2),
        io::get_str(best_move.0 .0),
        ('A' as u8 + best_move.0 .1 - 1) as char,
        best_move.0 .2,
        ('A' as u8 + best_move.1 .0 as u8 - 1) as char,
        best_move.1 .1
    );

    if best_move.0 .0 == 255 || io::is_checkmate(best_board.0, best_board.1, best_board.2, !white) {
        engine::GameState::CheckMate(best_board)
    } else {
        engine::GameState::Normal(best_board)
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
    board: engine::Board,
    black_figures: engine::Positions,
    white_figures: engine::Positions,
    white: bool,
    mut alpha: i32,
    mut beta: i32,
    depth: u8,
) -> i32 {
    if black_figures[0].0 == 255 {
        return i32::MAX - 100 + depth as i32;
    } else if white_figures[0].0 == 255 {
        return i32::MIN + 100 - depth as i32;
    }

    if depth == 0 {
        if {
            let mut could_catch_king = false;

            let king = if white {
                black_figures[0]
            } else {
                white_figures[0]
            };

            'search: for figure in {
                if white {
                    white_figures
                } else {
                    black_figures
                }
            } {
                if figure.0 == 255 {
                    continue;
                }

                let moves = engine::get_valid_moves(board, figure.0, figure.1, figure.2);

                for subset in moves {
                    for r#move in subset {
                        if r#move.0 > 8 {
                            break;
                        }

                        if r#move.0 as u8 == king.1 && r#move.1 as u8 == king.2 {
                            could_catch_king = true;
                            break 'search;
                        }
                    }
                }
            }

            could_catch_king
        } {
            return if white {
                i32::MAX - 100 + depth as i32
            } else {
                i32::MIN + 100 - depth as i32
            };
        }

        return evaluate_board(black_figures, white_figures);
    }

    let mut min = i32::MAX - 100 + depth as i32;
    let mut max = i32::MIN + 100 - depth as i32;

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

        let moves = engine::get_valid_moves(board, figure.0, figure.1, figure.2);

        for subset in moves {
            for r#move in subset {
                if r#move.0 > 8 {
                    break;
                }

                let (board, black_figures, white_figures) = engine::move_figure(
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

                if white && var == i32::MAX - 100 + depth as i32 - 1 {
                    return var;
                } else if !white && var == i32::MIN + 100 - depth as i32 - 1 {
                    return var;
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
pub fn evaluate_board(black_figures: engine::Positions, white_figures: engine::Positions) -> i32 {
    let mut mg_score = 0;
    let mut eg_score = 0;

    let mut game_phase = 0;

    for i in 0..black_figures.len() {
        let white = white_figures[i];
        let black = black_figures[i];

        if white.0 != 255 {
            mg_score += evaluation::MG_VALUES[white.0 as usize]
                + evaluation::MG_TABLES[white.0 as usize]
                    [(8 * (white.2 - 1) + (white.1 - 1)) as usize];
            eg_score += evaluation::EG_VALUES[white.0 as usize]
                + evaluation::EG_TABLES[white.0 as usize]
                    [(8 * (white.2 - 1) + (white.1 - 1)) as usize];

            game_phase += evaluation::GAME_PHASE_INC[white.0 as usize]
        }

        if black.0 != 255 {
            mg_score -= evaluation::MG_VALUES[(black.0 - 10) as usize]
                + evaluation::MG_TABLES[(black.0 - 10) as usize]
                    [((8 * (7 - (black.2 - 1)) + (black.1 - 1)) as usize)];
            eg_score -= evaluation::EG_VALUES[(black.0 - 10) as usize]
                + evaluation::EG_TABLES[(black.0 - 10) as usize]
                    [((8 * (7 - (black.2 - 1)) + (black.1 - 1)) as usize)];

            game_phase += evaluation::GAME_PHASE_INC[(black.0 - 10) as usize]
        }
    }

    let mg_phase = if game_phase > 24 { 24 } else { game_phase };
    let eg_phase = 24 - mg_phase;

    (mg_score * mg_phase + eg_score * eg_phase) / 24
}
