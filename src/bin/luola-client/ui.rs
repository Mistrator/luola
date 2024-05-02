use crate::terminal::canvas::Canvas;
use crate::ui::creature_info::CreatureInfo;
use crate::ui::inventory_info::InventoryInfo;
use crate::ui::message_log::MessageLog;
use crate::ui::viewport::Viewport;
use crate::GameState;

mod borders;
mod color_scheme;
mod creature_info;
mod inventory_info;
mod message_log;
mod viewport;

pub struct UI {
    width: usize,
    height: usize,

    pub viewport: Viewport,
    creature_info: CreatureInfo,
    pub inventory_info: InventoryInfo,
    pub message_log: MessageLog,

    displayed_creature: Option<u128>,
    default_displayed_creature: Option<u128>,
}

impl UI {
    pub fn new(width: usize, height: usize) -> Self {
        let sidebar_width = 32;
        let viewport_width = width - sidebar_width;
        let viewport_height = height / 4 * 3;
        let message_log_height = height - viewport_height;

        let mut inventory_info = InventoryInfo::new(sidebar_width - 2, height / 2 - 2);
        inventory_info.select_slot(0);

        Self {
            width,
            height,

            viewport: Viewport::new(viewport_width - 2, viewport_height - 2),
            creature_info: CreatureInfo::new(sidebar_width - 2, height / 2 - 2),
            inventory_info,
            message_log: MessageLog::new(viewport_width - 2, message_log_height - 2),

            displayed_creature: None,
            default_displayed_creature: None,
        }
    }

    pub fn render(state: &mut GameState) -> Canvas {
        let mut canvas = Canvas::new(state.ui.width, state.ui.height);

        let viewport = state.ui.viewport.render(state);
        let viewport = borders::add_rounded_borders(&viewport, color_scheme::BORDER_STYLE);
        canvas.paste(&viewport, 0, 0);

        let creature_info = state
            .ui
            .creature_info
            .render(state.ui.get_displayed_creature(), &state.layer);
        let creature_info =
            borders::add_rounded_borders(&creature_info, color_scheme::BORDER_STYLE);
        canvas.paste(&creature_info, 0, viewport.get_width());

        let inventory_info = state
            .ui
            .inventory_info
            .render(state.ui.get_displayed_creature(), &state.layer);
        let inventory_info =
            borders::add_rounded_borders(&inventory_info, color_scheme::BORDER_STYLE);
        canvas.paste(
            &inventory_info,
            creature_info.get_height(),
            viewport.get_width(),
        );

        let message_log = state.ui.message_log.render(&state.layer);
        let message_log = borders::add_rounded_borders(&message_log, color_scheme::BORDER_STYLE);
        canvas.paste(&message_log, viewport.get_height(), 0);

        canvas
    }

    pub fn get_displayed_creature(&self) -> Option<u128> {
        if self.displayed_creature.is_some() {
            return self.displayed_creature;
        }
        self.default_displayed_creature
    }

    pub fn set_displayed_creature(&mut self, creature: u128) {
        self.displayed_creature = Some(creature);
    }

    pub fn remove_displayed_creature(&mut self) {
        self.displayed_creature = None;
    }

    pub fn get_default_displayed_creature(&self) -> Option<u128> {
        self.default_displayed_creature
    }

    pub fn set_default_displayed_creature(&mut self, creature: u128) {
        self.default_displayed_creature = Some(creature);
    }
}
