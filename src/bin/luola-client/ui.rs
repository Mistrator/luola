use crate::terminal::canvas::Canvas;
use crate::ui::viewport::Viewport;
use luola::world::Layer;

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
            viewport: Viewport::new(width, height),
        }
    }

    pub fn render(&self, layer: &Layer) -> Canvas {
        self.viewport.render(layer)
    }
}
