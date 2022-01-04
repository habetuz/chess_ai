use crate::engine::figures::{
    Figure, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};

mod figures;

pub type Board = [(Figure, u8, u8); 32];

// Initial board.
pub const INITIAL_BOARD: Board = [
    (BLACK_ROOK, 1, 8),
    (BLACK_KNIGHT, 2, 8),
    (BLACK_BISHOP, 3, 8),
    (BLACK_QUEEN, 4, 8),
    (BLACK_KING, 5, 8),
    (BLACK_BISHOP, 6, 8),
    (BLACK_KNIGHT, 7, 8),
    (BLACK_ROOK, 8, 8),
    (BLACK_PAWN, 1, 7),
    (BLACK_PAWN, 2, 7),
    (BLACK_PAWN, 3, 7),
    (BLACK_PAWN, 4, 7),
    (BLACK_PAWN, 5, 7),
    (BLACK_PAWN, 6, 7),
    (BLACK_PAWN, 7, 7),
    (BLACK_PAWN, 8, 7),
    (WHITE_ROOK, 1, 1),
    (WHITE_KNIGHT, 2, 1),
    (WHITE_BISHOP, 3, 1),
    (WHITE_QUEEN, 4, 1),
    (WHITE_KING, 5, 1),
    (WHITE_BISHOP, 6, 1),
    (WHITE_KNIGHT, 7, 1),
    (WHITE_ROOK, 8, 1),
    (WHITE_PAWN, 1, 2),
    (WHITE_PAWN, 2, 2),
    (WHITE_PAWN, 3, 2),
    (WHITE_PAWN, 4, 2),
    (WHITE_PAWN, 5, 2),
    (WHITE_PAWN, 6, 2),
    (WHITE_PAWN, 7, 2),
    (WHITE_PAWN, 8, 2),
];

pub enum GameState {
    Normal(Board),
    CheckMate(Board),
    Check(Board),
}
