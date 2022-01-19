pub mod figures;

pub type Board = [u8; 66];

/// The last two u8 (index 64 and 65) contain if castling is still possible.
/// First digit (64): 0 if no castling is allowed, 1 if left castling is allowed, 2 if right castling is allowed and 3 if both castling are allowed for white.
/// Second digit (65): 0 if no castling is allowed, 1 if left castling is allowed, 2 if right castling is allowed and 3 if both castling are allowed for black.
pub static INITIAL_BOARD: Board = [
    figures::WHITE_ROOK,
    figures::WHITE_KNIGHT,
    figures::WHITE_BISHOP,
    figures::WHITE_QUEEN,
    figures::WHITE_KING,
    figures::WHITE_BISHOP,
    figures::WHITE_KNIGHT,
    figures::WHITE_ROOK,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    figures::WHITE_PAWN,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_PAWN,
    figures::BLACK_ROOK,
    figures::BLACK_KNIGHT,
    figures::BLACK_BISHOP,
    figures::BLACK_QUEEN,
    figures::BLACK_KING,
    figures::BLACK_BISHOP,
    figures::BLACK_KNIGHT,
    figures::BLACK_ROOK,
    3,
    3,
];

/// **Type Position**
///
/// ```
/// The Figure
///  |       x   y
///  |       |   |
/// (Figure, u8, u8)
/// ```
///
/// [`Figure`] will be `255` when it is caught.
pub type Positions = [Position; 16];
pub type Position = (figures::Figure, u8, u8);

pub static POSITIONS_WHITE: Positions = [
    (figures::WHITE_KING, 5, 1),
    (figures::WHITE_QUEEN, 4, 1),
    (figures::WHITE_KNIGHT, 2, 1),
    (figures::WHITE_KNIGHT, 7, 1),
    (figures::WHITE_BISHOP, 3, 1),
    (figures::WHITE_BISHOP, 6, 1),
    (figures::WHITE_ROOK, 1, 1),
    (figures::WHITE_ROOK, 8, 1),
    (figures::WHITE_PAWN, 1, 2),
    (figures::WHITE_PAWN, 2, 2),
    (figures::WHITE_PAWN, 3, 2),
    (figures::WHITE_PAWN, 4, 2),
    (figures::WHITE_PAWN, 5, 2),
    (figures::WHITE_PAWN, 6, 2),
    (figures::WHITE_PAWN, 7, 2),
    (figures::WHITE_PAWN, 8, 2),
];

pub static POSITIONS_BLACK: Positions = [
    (figures::BLACK_KING, 5, 8),
    (figures::BLACK_QUEEN, 4, 8),
    (figures::BLACK_KNIGHT, 2, 8),
    (figures::BLACK_KNIGHT, 7, 8),
    (figures::BLACK_BISHOP, 3, 8),
    (figures::BLACK_BISHOP, 6, 8),
    (figures::BLACK_ROOK, 1, 8),
    (figures::BLACK_ROOK, 8, 8),
    (figures::BLACK_PAWN, 1, 7),
    (figures::BLACK_PAWN, 2, 7),
    (figures::BLACK_PAWN, 3, 7),
    (figures::BLACK_PAWN, 4, 7),
    (figures::BLACK_PAWN, 5, 7),
    (figures::BLACK_PAWN, 6, 7),
    (figures::BLACK_PAWN, 7, 7),
    (figures::BLACK_PAWN, 8, 7),
];

/*
pub static INITIAL_BOARD: Board = [
    0,
    0,
    0,
    0,
    figures::WHITE_KING,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    figures::BLACK_QUEEN,
    figures::BLACK_KING,
    0,
    0,
    0,
    0,
    0,
];

/// **Type Position**
///
/// ```
/// The Figure
///  |       x   y
///  |       |   |
/// (Figure, u8, u8)
/// ```
///
/// [`Figure`] will be `255` when it is caught.
pub type Positions = [Position; 16];
pub type Position = (figures::Figure, u8, u8);

pub static POSITIONS_WHITE: Positions = [
    (figures::WHITE_KING, 5, 1),
    (255, 4, 1),
    (255, 2, 1),
    (255, 7, 1),
    (255, 3, 1),
    (255, 6, 1),
    (255, 1, 1),
    (255, 8, 1),
    (255, 1, 2),
    (255, 2, 2),
    (255, 3, 2),
    (255, 4, 2),
    (255, 5, 2),
    (255, 6, 2),
    (255, 7, 2),
    (255, 8, 2),
];

pub static POSITIONS_BLACK: Positions = [
    (figures::BLACK_KING, 5, 8),
    (figures::BLACK_QUEEN, 4, 8),
    (255, 2, 8),
    (255, 7, 8),
    (255, 3, 8),
    (255, 6, 8),
    (255, 1, 8),
    (255, 8, 8),
    (255, 1, 7),
    (255, 2, 7),
    (255, 3, 7),
    (255, 4, 7),
    (255, 5, 7),
    (255, 6, 7),
    (255, 7, 7),
    (255, 8, 7),
];

 */

pub enum GameState {
    Normal((Board, Positions, Positions)),
    CheckMate((Board, Positions, Positions)),
}

pub fn get_figure(board: Board, x: u8, y: u8) -> figures::Figure {
    board[(8 * (y - 1) + (x - 1)) as usize]
}

pub fn set_figure(mut board: Board, figure: figures::Figure, x: u8, y: u8) -> Board {
    board[(8 * (y - 1) + (x - 1)) as usize] = figure;
    board
}

pub fn get_valid_moves(
    board: Board,
    figure: figures::Figure,
    x: u8,
    y: u8,
) -> figures::MovementSet {
    let mut moves = figures::get_relative_moves(figure, y);
    for i in 0..moves.len() {
        let mut subset = moves[i];
        if subset[0].0 > 8 {
            break;
        }

        let mut leave = false;
        for i in 0..subset.len() {
            let mut position = subset[i];

            // Make absolute
            position.0 += x as i8;
            position.1 += y as i8;

            if leave || position.0 > 8 || position.1 > 8 || position.0 < 1 || position.1 < 1 {
                position.0 = 127;
                position.1 = 127;
                subset[i] = position;
                break;
            }

            let figure_at_move = get_figure(board, position.0 as u8, position.1 as u8);

            // Check if the figure at the position is the opposite color (and can therefore be caught)
            leave = figure_at_move != 0;

            // Check if the figure at the position is the same color
            if leave && figure_at_move / 10 == figure / 10 {
                position.0 = 127;
                position.1 = 127;
                subset[i] = position;
                break;
            }

            // Special pawn rule: It can only move diagonally if it can catch a figure there and it cannot move forward if would catch a figure there.
            if (figure == figures::WHITE_PAWN || figure == figures::BLACK_PAWN)
                && ((position.0 as u8 != x && !leave) || (position.0 as u8 == x && leave))
            {
                position.0 = 127;
                position.1 = 127;
                subset[i] = position;
                break;
            // Special king/rook rule: Castling. See https://schach.de/de/page/schachregeln-die-rochade
            } else if figure == figures::WHITE_KING || figure == figures::BLACK_KING {
                if position.0 == x as i8 - 2 {
                    if !((board[(64 + figure / 10) as usize] == 1
                        || board[(64 + figure / 10) as usize] == 3)
                        && get_figure(board, (position.0 - 1) as u8, position.1 as u8) == 0)
                    {
                        position.0 = 127;
                        position.1 = 127;
                        subset[i] = position;
                        break;
                    }
                } else if position.0 == x as i8 + 2 {
                    if board[(64 + figure / 10) as usize] < 2 {
                        position.0 = 127;
                        position.1 = 127;
                        subset[i] = position;
                        break;
                    }
                }
            }

            subset[i] = position;
        }
        moves[i] = subset;
    }
    moves
}

pub fn move_figure(
    mut board: Board,
    mut black_figures: Positions,
    mut white_figures: Positions,
    mut figure: figures::Figure,
    from_x: u8,
    from_y: u8,
    to_x: u8,
    to_y: u8,
    white: bool,
) -> (Board, Positions, Positions) {
    board = set_figure(board, 0, from_x, from_y);

    let caught_figure = get_figure(board, to_x, to_y);

    // Check if the move catches a figure
    if caught_figure != 0 {
        // Mark that figure as caught
        if white {
            for i in (caught_figure - 10 - 1) as usize..black_figures.len() {
                let mut figure = black_figures[i as usize];
                if figure.0 != 255 && figure.1 == to_x && figure.2 == to_y {
                    figure.0 = 255;
                    black_figures[i] = figure;
                }
            }
        } else {
            for i in (caught_figure - 1) as usize..white_figures.len() {
                let mut figure = white_figures[i];
                if figure.0 != 255 && figure.1 == to_x && figure.2 == to_y {
                    figure.0 = 255;
                    white_figures[i] = figure;
                }
            }
        }
    }

    // Update position of figure
    if white {
        for i in (figure - 1) as usize..white_figures.len() {
            let mut figure = white_figures[i];
            if figure.1 == from_x && figure.2 == from_y {
                if figure.0 == figures::WHITE_PAWN && to_y == 8 {
                    figure.0 = figures::WHITE_QUEEN;
                }
                figure.1 = to_x;
                figure.2 = to_y;
            }

            white_figures[i] = figure;
        }
    } else {
        for i in (figure - 10 - 1) as usize..black_figures.len() {
            let mut figure = black_figures[i];
            if figure.1 == from_x && figure.2 == from_y {
                if figure.0 == figures::BLACK_PAWN && to_y == 1 {
                    figure.0 = figures::BLACK_QUEEN;
                }
                figure.1 = to_x;
                figure.2 = to_y;
            }

            black_figures[i] = figure;
        }
    }

    // Special rule castling
    if (figures::colored_figure_to_blank_figure(figure) == figures::KING) && from_x == 5 {
        if to_x == from_x - 2 {
            let update = move_figure(
                board,
                black_figures,
                white_figures,
                get_figure(board, 1, from_y),
                1,
                from_y,
                4,
                from_y,
                white,
            );
            board = update.0;
            black_figures = update.1;
            white_figures = update.2;
        } else if to_x == from_x + 2 {
            let update = move_figure(
                board,
                black_figures,
                white_figures,
                get_figure(board, 8, from_y),
                8,
                from_y,
                6,
                from_y,
                white,
            );
            board = update.0;
            black_figures = update.1;
            white_figures = update.2;
        }
        board[(64 + figure / 10) as usize] = 0;
    } else if figures::colored_figure_to_blank_figure(figure) == figures::ROOK {
        if (board[(64 + figure / 10) as usize] == 1 || board[(64 + figure / 10) as usize] == 3)
            && from_x == 1
        {
            board[(64 + figure / 10) as usize] -= 1
        } else if board[(64 + figure / 10) as usize] >= 2 && from_x == 8 {
            board[(64 + figure / 10) as usize] -= 2
        }
    }

    if figure == figures::WHITE_PAWN && to_y == 8 {
        figure = figures::WHITE_QUEEN;
    } else if figure == figures::BLACK_PAWN && to_y == 1 {
        figure = figures::BLACK_PAWN;
    }

    board = set_figure(board, figure, to_x, to_y);

    (board, black_figures, white_figures)
}

pub fn contains_position(movement_set: figures::MovementSet, position: (u8, u8)) -> bool {
    for subset in movement_set {
        for r#move in subset {
            if r#move.0 > 8 {
                break;
            }
            if r#move.0 == position.0 as i8 && r#move.1 == position.1 as i8 {
                return true;
            }
        }
    }
    false
}

pub fn is_board_valid(
    board: Board,
    black_figures: Positions,
    white_figures: Positions,
    white: bool,
) -> bool {
    let king = {
        if white {
            white_figures[0]
        } else {
            black_figures[0]
        }
    };

    for figure in {
        if white {
            black_figures
        } else {
            white_figures
        }
    } {
        if figure.0 == 255 {
            continue;
        }

        let movement_set = get_valid_moves(board, figure.0, figure.1, figure.2);
        if contains_position(movement_set, (king.1, king.2)) {
            return false;
        }
    }
    true
}
