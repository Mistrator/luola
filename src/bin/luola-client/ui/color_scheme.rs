use crate::terminal::color::Color;
use crate::terminal::styled_char::Style;

const WORLD_BACKGROUND_COLOR: Color = Color::Black;
const WORLD_WALL_COLOR: Color = Color::White;

const WIDGET_BACKGROUND_COLOR: Color = Color::Black;
const BORDER_COLOR: Color = Color::White;

const TEXT_COLOR: Color = Color::White;
const TEXT_HIGHLIGHT_COLOR: Color = Color::BrightWhite;
const TEXT_ERROR_COLOR: Color = Color::BrightYellow;

const GOOD_MODIFIER_COLOR: Color = Color::Green;
const BAD_MODIFIER_COLOR: Color = Color::Red;

const SELECTION_COLOR: Color = Color::BrightCyan;

const CRITICAL_SUCCESS_COLOR: Color = Color::BrightGreen;
const SUCCESS_COLOR: Color = Color::Green;
const FAILURE_COLOR: Color = Color::Red;
const CRITICAL_FAILURE_COLOR: Color = Color::BrightRed;

pub const WORLD_EMPTY_STYLE: Style = Style {
    foreground_color: Color::Transparent,
    background_color: WORLD_BACKGROUND_COLOR,
};

pub const WORLD_WALL_STYLE: Style = Style {
    foreground_color: Color::Transparent,
    background_color: WORLD_WALL_COLOR,
};

pub const BORDER_STYLE: Style = Style {
    foreground_color: BORDER_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const TEXT_STYLE: Style = Style {
    foreground_color: TEXT_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const TEXT_HIGHLIGHT_STYLE: Style = Style {
    foreground_color: TEXT_HIGHLIGHT_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const TEXT_ERROR_STYLE: Style = Style {
    foreground_color: TEXT_ERROR_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const GOOD_MODIFIER_STYLE: Style = Style {
    foreground_color: GOOD_MODIFIER_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const BAD_MODIFIER_STYLE: Style = Style {
    foreground_color: BAD_MODIFIER_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const SELECTION_STYLE: Style = Style {
    foreground_color: SELECTION_COLOR,
    background_color: Color::Transparent,
};

pub const CRITICAL_SUCCESS_STYLE: Style = Style {
    foreground_color: CRITICAL_SUCCESS_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const SUCCESS_STYLE: Style = Style {
    foreground_color: SUCCESS_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const FAILURE_STYLE: Style = Style {
    foreground_color: FAILURE_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const CRITICAL_FAILURE_STYLE: Style = Style {
    foreground_color: CRITICAL_FAILURE_COLOR,
    background_color: WIDGET_BACKGROUND_COLOR,
};

pub const PLAYER_CREATURE_STYLE: Style = Style {
    foreground_color: Color::Blue,
    background_color: WORLD_BACKGROUND_COLOR,
};

pub const NONPLAYER_CREATURE_STYLE: Style = Style {
    foreground_color: Color::Red,
    background_color: WORLD_BACKGROUND_COLOR,
};

pub const ACTIVE_PLAYER_CREATURE_STYLE: Style = Style {
    foreground_color: Color::BrightBlue,
    background_color: WORLD_BACKGROUND_COLOR,
};

pub const ACTIVE_NONPLAYER_CREATURE_STYLE: Style = Style {
    foreground_color: Color::BrightRed,
    background_color: WORLD_BACKGROUND_COLOR,
};

pub const BULLET_POINT: &str = "\u{25ba}";
