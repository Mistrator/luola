use crate::terminal::canvas::Canvas;
use crate::ui::creature_info::CreatureInfo;
use crate::ui::viewport::Viewport;
use luola::world::Layer;

mod borders;
mod color_scheme;
mod creature_info;
mod viewport;

pub struct UI {
    width: usize,
    height: usize,
    sidebar_width: usize,

    viewport: Viewport,
    creature_info: CreatureInfo,

    selected_creature: Option<u128>,
}

impl UI {
    pub fn new(width: usize, height: usize) -> Self {
        let sidebar_width = 64;
        let viewport_width = width - sidebar_width;

        Self {
            width,
            height,
            sidebar_width,

            viewport: Viewport::new(viewport_width - 2, height - 2),
            creature_info: CreatureInfo::new(sidebar_width - 2, height - 2),

            selected_creature: None,
        }
    }

    pub fn render(&self, layer: &Layer) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        let viewport = self.viewport.render(layer);
        let viewport = borders::add_rounded_borders(&viewport, color_scheme::BORDER_STYLE);
        canvas.paste(&viewport, 0, 0);

        let creature_info = self.creature_info.render(self.selected_creature, layer);
        let creature_info =
            borders::add_rounded_borders(&creature_info, color_scheme::BORDER_STYLE);
        canvas.paste(&creature_info, 0, viewport.get_width());

        canvas
    }

    pub fn select_creature(&mut self, creature_id: u128) {
        self.selected_creature = Some(creature_id);
    }

    pub fn deselect_creature(&mut self) {
        self.selected_creature = None;
    }
}
