use crate::engine::figures;
use colored;
use colored::Colorize;

/// Konvertiert die gegebene [`figures::Figure`] zu einem [`colored::ColoredString`].
///
/// # Parameter
/// * `figure`: Die [`figures::Figure`], die konvertiert werden soll.
///
/// # Rückgabewert `colored::ColoredString`
/// Gibt die Figur als [`colored::ColoredString`] mit schwarzem Vordergrund und weißem Hintergrund zurück.
pub fn get_str(figure: figures::Figure) -> colored::ColoredString {
    let r#char = match figure {
        figures::WHITE_PAWN => figures::CHAR_WHITE_PAWN,
        figures::WHITE_KING => figures::CHAR_WHITE_KING,
        figures::WHITE_ROOK => figures::CHAR_WHITE_ROOK,
        figures::WHITE_QUEEN => figures::CHAR_WHITE_QUEEN,
        figures::WHITE_KNIGHT => figures::CHAR_WHITE_KNIGHT,
        figures::WHITE_BISHOP => figures::CHAR_WHITE_BISHOP,
        figures::BLACK_PAWN => figures::CHAR_BLACK_PAWN,
        figures::BLACK_KING => figures::CHAR_BLACK_KING,
        figures::BLACK_ROOK => figures::CHAR_BLACK_ROOK,
        figures::BLACK_QUEEN => figures::CHAR_BLACK_QUEEN,
        figures::BLACK_KNIGHT => figures::CHAR_BLACK_KNIGHT,
        figures::BLACK_BISHOP => figures::CHAR_BLACK_BISHOP,
        _ => ' ',
    };

    format!(" {}", r#char).black().on_white()
}
