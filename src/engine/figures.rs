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

const WHITE: Figure = 0;
const BLACK: Figure = 10;

pub const KING: Figure = 1;
pub const QUEEN: Figure = 2;
pub const KNIGHT: Figure = 3;
pub const BISHOP: Figure = 5;
pub const ROOK: Figure = 7;
pub const PAWN: Figure = 9;

pub const WHITE_KING: Figure = KING + WHITE;
pub const WHITE_QUEEN: Figure = QUEEN + WHITE;
pub const WHITE_KNIGHT: Figure = KNIGHT + WHITE;
pub const WHITE_BISHOP: Figure = BISHOP + WHITE;
pub const WHITE_ROOK: Figure = ROOK + WHITE;
pub const WHITE_PAWN: Figure = PAWN + WHITE;

pub const BLACK_KING: Figure = KING + BLACK;
pub const BLACK_QUEEN: Figure = QUEEN + BLACK;
pub const BLACK_KNIGHT: Figure = KNIGHT + BLACK;
pub const BLACK_BISHOP: Figure = BISHOP + BLACK;
pub const BLACK_ROOK: Figure = ROOK + BLACK;
pub const BLACK_PAWN: Figure = PAWN + BLACK;

static MOVEMENTS: [MovementSet; 21] = [
    [[(0, 0); 7]; 8],
    MOVEMENT_KING,
    MOVEMENT_QUEEN,
    MOVEMENT_KNIGHT,
    [[(0, 0); 7]; 8],
    MOVEMENT_BISHOP,
    [[(0, 0); 7]; 8],
    MOVEMENT_ROOK,
    [[(0, 0); 7]; 8],
    MOVEMENT_WHITE_PAWN_MOVED,
    MOVEMENT_WHITE_PAWN_UNMOVED,
    MOVEMENT_KING,
    MOVEMENT_QUEEN,
    MOVEMENT_KNIGHT,
    [[(0, 0); 7]; 8],
    MOVEMENT_BISHOP,
    [[(0, 0); 7]; 8],
    MOVEMENT_ROOK,
    [[(0, 0); 7]; 8],
    MOVEMENT_BLACK_PAWN_MOVED,
    MOVEMENT_BLACK_PAWN_UNMOVED,
];

const MOVEMENT_KNIGHT: MovementSet = [
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

const MOVEMENT_BISHOP: MovementSet = [
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

const MOVEMENT_ROOK: MovementSet = [
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

const MOVEMENT_QUEEN: MovementSet = [
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

const MOVEMENT_KING: MovementSet = [
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

const MOVEMENT_WHITE_PAWN_UNMOVED: MovementSet = [
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

const MOVEMENT_WHITE_PAWN_MOVED: MovementSet = [
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

const MOVEMENT_BLACK_PAWN_UNMOVED: MovementSet = [
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

const MOVEMENT_BLACK_PAWN_MOVED: MovementSet = [
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

pub fn get_relative_moves(mut figure: Figure, y: u8) -> MovementSet {
    if figure == WHITE_PAWN && y == 2 {
        figure += 1;
    } else if figure == BLACK_PAWN && y == 7 {
        figure += 1;
    }

    MOVEMENTS[figure as usize]
}

pub fn colored_figure_to_blank_figure(figure: Figure) -> Figure {
    figure - 10 * (figure / 10)
}
