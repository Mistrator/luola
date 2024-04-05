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

    pub fn get_ansi_sequences(&self) -> String {
        let foreground = ansi_sequences::set_foreground_color(self.foreground_color);
        let background = ansi_sequences::set_background_color(self.background_color);

        format!("{}{}", foreground, background)
    }
}

#[derive(Clone, Copy)]
pub struct StyledChar {
    content: char,
    style: Style,
}

impl StyledChar {
    pub fn new_default() -> Self {
        Self {
            content: ' ',
            style: Style::default_style(),
        }
    }

    pub fn new(content: char, style: Style) -> Self {
        Self { content, style }
    }

    pub fn render(&self) -> String {
        let ansi_style = self.style.get_ansi_sequences();
        format!("{}{}", ansi_style, self.content.to_string())
    }
}
