// outer is 1-8 (y)
// inner is A-H (x)
type board = [[char; 8]; 8];

// TODO: implement
// TODO: should print flip the board if it's black turn?
pub fn print_board(b: board) {
    println!("{}", b)
}

// new_game returns a board with the
// pieces in the default setup
// if white_handycap is true, white has one
// pawn less to begin with
pub fn new_game(white_handicap: bool) -> board {
    let b: board = [
        [
            black_rook,
            black_knight,
            black_bishop,
            black_queen,
            black_king,
            black_bishop,
            black_knight,
            black_rook,
        ],
        [black_pawn; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [0; 8],
        [white_pawn; 8],
        [
            white_rook,
            white_knight,
            white_bishop,
            white_queen,
            white_king,
            white_bishop,
            white_knight,
            white_rook,
        ],
    ];
    if white_handicap {
        b[6][0] = 0;
    }
    return b;
}
