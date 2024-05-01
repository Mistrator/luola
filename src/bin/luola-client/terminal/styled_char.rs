use crate::terminal::ansi_sequences;
use crate::terminal::color::Color;

#[derive(Clone, Copy)]
pub struct Style {
    pub foreground_color: Color,
    pub background_color: Color,
}

impl Style {
    pub fn default_style() -> Self {
        Self {
            foreground_color: Color::White,
            background_color: Color::Black,
        }
    }

    pub fn transparent_style() -> Self {
        Self {
            foreground_color: Color::Transparent,
            background_color: Color::Transparent,
        }
    }

    pub fn get_ansi_sequences(&self) -> String {
        let foreground = ansi_sequences::set_foreground_color(self.foreground_color);
        let background = ansi_sequences::set_background_color(self.background_color);

        format!("{}{}", foreground, background)
    }

    pub fn color_passthrough(upper: Style, lower: Style) -> Style {
        let mut new_style = upper;

        if new_style.foreground_color == Color::Transparent {
            new_style.foreground_color = lower.foreground_color;
        }
        if new_style.background_color == Color::Transparent {
            new_style.background_color = lower.background_color;
        }

        new_style
    }
}

#[derive(Clone, Copy)]
pub struct StyledChar {
    content: Option<char>,
    pub style: Style,
}

impl StyledChar {
    pub fn new_default() -> Self {
        Self {
            content: None,
            style: Style::default_style(),
        }
    }

    pub fn new_transparent() -> Self {
        Self {
            content: None,
            style: Style::transparent_style(),
        }
    }

    pub fn new_empty(style: Style) -> Self {
        Self {
            content: None,
            style,
        }
    }

    pub fn new(content: char, style: Style) -> Self {
        Self {
            content: Some(content),
            style,
        }
    }

    pub fn render(&self) -> String {
        let ansi_style = self.style.get_ansi_sequences();

        match self.content {
            Some(c) => format!("{}{}", ansi_style, c.to_string()),
            None => format!("{} ", ansi_style),
        }
    }

    pub fn merge(upper: Self, lower: Self) -> Self {
        let content = match upper.content {
            Some(_) => upper.content,
            None => lower.content,
        };

        let style = Style::color_passthrough(upper.style, lower.style);

        Self { content, style }
    }
}
