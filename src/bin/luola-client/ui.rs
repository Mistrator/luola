use crate::terminal::canvas::Canvas;
use crate::terminal::color::Color;
use crate::terminal::styled_char::Style;
use crate::ui::viewport::Viewport;
use luola::world::Layer;

mod borders;
mod viewport;

pub struct UI {
    pub width: usize,
    pub height: usize,

    viewport: Viewport,
}

impl UI {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            viewport: Viewport::new(width - 2, height - 2),
        }
    }

    pub fn render(&self, layer: &Layer) -> Canvas {
        let viewport = self.viewport.render(layer);

        let border_style = Style {
            foreground_color: Color::White,
            background_color: Color::Black,
        };

        borders::add_rounded_borders(&viewport, border_style)
    }
}
