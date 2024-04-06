use crate::terminal::color::Color;

fn prefix() -> String {
    String::from("\x1b[")
}

pub fn clear_screen() -> String {
    format!("{}2J", prefix())
}

pub fn use_alternate_screen_buffer() -> String {
    format!("{}?1049h", prefix())
}

pub fn use_main_screen_buffer() -> String {
    format!("{}?1049l", prefix())
}

pub fn set_cursor_position(row: usize, column: usize) -> String {
    assert!(row >= 1 && column >= 1, "indexing is 1-based");

    format!("{}{};{}H", prefix(), row, column)
}

fn format_text(sgr: String) -> String {
    format!("{}{}m", prefix(), sgr)
}

fn color_str(color: Color) -> String {
    match color {
        Color::Black => String::from("0"),
        Color::Red => String::from("1"),
        Color::Green => String::from("2"),
        Color::Yellow => String::from("3"),
        Color::Blue => String::from("4"),
        Color::Magenta => String::from("5"),
        Color::Cyan => String::from("6"),
        Color::White => String::from("7"),
        Color::RGB(r, g, b) => format!("8;2;{};{};{}", r, g, b),
        Color::Transparent => panic!("must have a color"),
    }
}

pub fn set_foreground_color(color: Color) -> String {
    let sgr = format!("3{}", color_str(color));
    format_text(sgr)
}

pub fn set_background_color(color: Color) -> String {
    let sgr = format!("4{}", color_str(color));
    format_text(sgr)
}
