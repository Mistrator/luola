use crate::terminal::color::Color;
use crate::terminal::styled_char::Style;

const BACKGROUND_COLOR: Color = Color::Black;
const BORDER_COLOR: Color = Color::White;

const TEXT_COLOR: Color = Color::White;
const TEXT_HIGHLIGHT_COLOR: Color = Color::BrightWhite;

const GOOD_MODIFIER_COLOR: Color = Color::Green;
const BAD_MODIFIER_COLOR: Color = Color::Red;

pub const BORDER_STYLE: Style = Style {
    foreground_color: BORDER_COLOR,
    background_color: BACKGROUND_COLOR,
};

pub const TEXT_STYLE: Style = Style {
    foreground_color: TEXT_COLOR,
    background_color: BACKGROUND_COLOR,
};

pub const TEXT_HIGHLIGHT_STYLE: Style = Style {
    foreground_color: TEXT_HIGHLIGHT_COLOR,
    background_color: BACKGROUND_COLOR,
};

pub const GOOD_MODIFIER_STYLE: Style = Style {
    foreground_color: GOOD_MODIFIER_COLOR,
    background_color: BACKGROUND_COLOR,
};

pub const BAD_MODIFIER_STYLE: Style = Style {
    foreground_color: BAD_MODIFIER_COLOR,
    background_color: BACKGROUND_COLOR,
};
