// pub type Figure = char;
pub type Figure = u8;

/// Contains all moves a figure can make.
/// `(119, 119)` and `[(119, 119); 7]` are filler moves to fill the array to the right size. They should be ignored.
///
/// A movement set contains multiple movement sub sets.
/// Movement sub sets are independent of each other
/// but if a figure cannot make one move of a movement sub set,
/// it also cannot do any of the following moves of that movement sub set.
///
/// **Example**
/// ```
/// [
///     [
///     (1, 1),
///     (2, 2),
///     (3, 3),
///     ...
///     ],
///     [
///     (-1, -1),
///     (-2, -2),
///     (-3, -3),
///     ...
///     ]
/// ]
/// ```
/// Let's say the figure can move `(1, 1)`, but not `(2, 2)`. Then only `(1, 1)` is a possible move of that sub set.
/// `(3, 3)` is not possible, because could only be possible if `(2, 2)` is possible.
///
/// The next sub set `(-1, -1), (-2, -2), (-3, -3)` could still be possible.
pub type MovementSet = [MovementSubSet; 8];
pub type MovementSubSet = [(i8, i8); 7];

pub const CHAR_WHITE_PAWN: char = '♙';
pub const CHAR_WHITE_KNIGHT: char = '♘';
pub const CHAR_WHITE_BISHOP: char = '♗';
pub const CHAR_WHITE_ROOK: char = '♖';
pub const CHAR_WHITE_QUEEN: char = '♕';
pub const CHAR_WHITE_KING: char = '♔';

pub const CHAR_BLACK_PAWN: char = '♟';
pub const CHAR_BLACK_KNIGHT: char = '♞';
pub const CHAR_BLACK_BISHOP: char = '♝';
pub const CHAR_BLACK_ROOK: char = '♜';
pub const CHAR_BLACK_QUEEN: char = '♛';
pub const CHAR_BLACK_KING: char = '♚';

pub const WHITE_KING: Figure = 11;
pub const WHITE_QUEEN: Figure = 12;
pub const WHITE_KNIGHT: Figure = 13;
pub const WHITE_BISHOP: Figure = 15;
pub const WHITE_ROOK: Figure = 17;
pub const WHITE_PAWN: Figure = 19;

pub const BLACK_KING: Figure = 101;
pub const BLACK_QUEEN: Figure = 102;
pub const BLACK_KNIGHT: Figure = 103;
pub const BLACK_BISHOP: Figure = 105;
pub const BLACK_ROOK: Figure = 107;
pub const BLACK_PAWN: Figure = 109;

static MOVEMENT_KNIGHT: MovementSet = [
    [
        (2, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (2, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, 2),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, 2),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-2, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-2, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, -2),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, -2),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
];

static MOVEMENT_BISHOP: MovementSet = [
    [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
    [
        (1, -1),
        (2, -2),
        (3, -3),
        (4, -4),
        (5, -5),
        (6, -6),
        (7, -7),
    ],
    [
        (-1, 1),
        (-2, 2),
        (-3, 3),
        (-4, 4),
        (-5, 5),
        (-6, 6),
        (-7, 7),
    ],
    [
        (-1, -1),
        (-2, -2),
        (-3, -3),
        (-4, -4),
        (-5, -5),
        (-6, -6),
        (-7, -7),
    ],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
];

static MOVEMENT_ROOK: MovementSet = [
    [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
    [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)],
    [
        (-1, 0),
        (-2, 0),
        (-3, 0),
        (-4, 0),
        (-5, 0),
        (-6, 0),
        (-7, 0),
    ],
    [
        (0, -1),
        (0, -2),
        (0, -3),
        (0, -4),
        (0, -5),
        (0, -6),
        (0, -7),
    ],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
];

static MOVEMENT_QUEEN: MovementSet = [
    [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
    [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
    [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)],
    [
        (1, -1),
        (2, -2),
        (3, -3),
        (4, -4),
        (5, -5),
        (6, -6),
        (7, -7),
    ],
    [
        (-1, 1),
        (-2, 2),
        (-3, 3),
        (-4, 4),
        (-5, 5),
        (-6, 6),
        (-7, 7),
    ],
    [
        (-1, -1),
        (-2, -2),
        (-3, -3),
        (-4, -4),
        (-5, -5),
        (-6, -6),
        (-7, -7),
    ],
    [
        (-1, 0),
        (-2, 0),
        (-3, 0),
        (-4, 0),
        (-5, 0),
        (-6, 0),
        (-7, 0),
    ],
    [
        (0, -1),
        (0, -2),
        (0, -3),
        (0, -4),
        (0, -5),
        (0, -6),
        (0, -7),
    ],
];

static MOVEMENT_KING: MovementSet = [
    [
        (1, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, 0),
        (2, 0),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (0, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, 0),
        (-2, 0),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (0, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
];

static MOVEMENT_WHITE_PAWN_UNMOVED: MovementSet = [
    [
        (0, 1),
        (0, 2),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
];

static MOVEMENT_WHITE_PAWN_MOVED: MovementSet = [
    [
        (0, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, 1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
];

static MOVEMENT_BLACK_PAWN_UNMOVED: MovementSet = [
    [
        (0, -1),
        (0, -2),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
];

static MOVEMENT_BLACK_PAWN_MOVED: MovementSet = [
    [
        (0, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (1, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [
        (-1, -1),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
        (119, 119),
    ],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
    [(119, 119); 7],
];

pub fn get_relative_moves(figure: Figure, y: u8) -> MovementSet {
    match figure {
        WHITE_KNIGHT => MOVEMENT_KNIGHT,
        BLACK_KNIGHT => MOVEMENT_KNIGHT,
        WHITE_BISHOP => MOVEMENT_BISHOP,
        BLACK_BISHOP => MOVEMENT_BISHOP,
        WHITE_QUEEN => MOVEMENT_QUEEN,
        BLACK_QUEEN => MOVEMENT_QUEEN,
        WHITE_KING => MOVEMENT_KING,
        BLACK_KING => MOVEMENT_KING,
        WHITE_ROOK => MOVEMENT_ROOK,
        BLACK_ROOK => MOVEMENT_ROOK,
        WHITE_PAWN => {
            if y == 2 {
                MOVEMENT_WHITE_PAWN_UNMOVED
            } else {
                MOVEMENT_WHITE_PAWN_MOVED
            }
        }
        BLACK_PAWN => {
            if y == 7 {
                MOVEMENT_BLACK_PAWN_UNMOVED
            } else {
                MOVEMENT_BLACK_PAWN_MOVED
            }
        }
        _ => panic!("That figure does not exist!"),
    }
}
