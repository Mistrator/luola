#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8, u8, u8),
    Transparent,
}

pub fn is_bright(color: Color) -> bool {
    color == Color::BrightBlack
        || color == Color::BrightRed
        || color == Color::BrightGreen
        || color == Color::BrightYellow
        || color == Color::BrightBlue
        || color == Color::BrightMagenta
        || color == Color::BrightCyan
        || color == Color::BrightWhite
}
