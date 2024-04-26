use crate::terminal::canvas::Canvas;
use crate::terminal::styled_char::Style;
use crate::ui::color_scheme;
use luola::check::Outcome;
use luola::info_message::*;
use luola::world::Layer;

pub struct MessageLog {
    width: usize,
    height: usize,

    // Newest message last, oldest first
    messages: Vec<MessageType>,
}

impl MessageLog {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: MessageType) {
        self.messages.push(message);
    }

    pub fn render(&mut self, layer: &Layer) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        let mut lines_rendered: usize = 0;
        let mut last_msg_rendered: Option<usize> = None;

        for i in (0..self.messages.len()).rev() {
            let msg = &self.messages[i];
            let rendered = self.render_message(msg, layer);

            let msg_lines = rendered.get_cursor_row() + 1;
            if lines_rendered + msg_lines > self.height {
                break;
            }

            let msg_start_row = self.height - lines_rendered - msg_lines;

            lines_rendered += msg_lines;
            last_msg_rendered = Some(i);

            canvas.paste_intersection(&rendered, msg_start_row, 0);
        }

        // Remove messages that do not fit in the log anymore
        // starting from the oldest
        if let Some(last) = last_msg_rendered {
            self.messages.drain(0..last);
        }

        canvas
    }

    fn render_message(&self, message: &MessageType, layer: &Layer) -> Canvas {
        match message {
            MessageType::Info(msg) => self.render_info_message(msg),
            MessageType::Error(msg) => self.render_error_message(msg),
            MessageType::Attack(msg) => self.render_attack_message(msg, layer),
        }
    }

    fn render_info_message(&self, message: &String) -> Canvas {
        self.render_single_style_message(message, color_scheme::TEXT_STYLE)
    }

    fn render_error_message(&self, message: &String) -> Canvas {
        self.render_single_style_message(message, color_scheme::TEXT_ERROR_STYLE)
    }

    fn render_single_style_message(&self, message: &String, style: Style) -> Canvas {
        let mut canvas = Canvas::new_transparent(self.width, self.height);
        canvas.write(message.clone(), style);

        canvas
    }

    fn render_attack_message(&self, message: &AttackMessage, layer: &Layer) -> Canvas {
        let mut canvas = Canvas::new_transparent(self.width, self.height);

        let attacker = layer.creatures.get(&message.attacker).unwrap();
        let item = layer.items.get(&message.item).unwrap();

        canvas.write(
            format!("{} ", attacker.name),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        canvas.write(
            format!("attacks with {}", item.name),
            color_scheme::TEXT_STYLE,
        );
        canvas.write_newline();

        for result in &message.results {
            let target = layer.creatures.get(&result.target).unwrap();
            let roll_total = result.check.roll + result.check.modifier;

            canvas.write(
                format!("{} {}: ", color_scheme::BULLET_POINT, target.name),
                color_scheme::TEXT_HIGHLIGHT_STYLE,
            );

            canvas.write(String::from("roll "), color_scheme::TEXT_STYLE);
            canvas.write(
                format!("{} ", roll_total),
                color_scheme::TEXT_HIGHLIGHT_STYLE,
            );

            canvas.write(String::from("(["), color_scheme::TEXT_STYLE);

            let roll_style = match result.check.roll {
                20 => color_scheme::CRITICAL_SUCCESS_STYLE,
                1 => color_scheme::CRITICAL_FAILURE_STYLE,
                _ => color_scheme::TEXT_HIGHLIGHT_STYLE,
            };

            canvas.write(format!("{}", result.check.roll), roll_style);
            canvas.write(
                format!("]{:+}), ", result.check.modifier),
                color_scheme::TEXT_STYLE,
            );

            match result.check.outcome {
                Outcome::CriticalSuccess => {
                    canvas.write(
                        String::from("critical success"),
                        color_scheme::CRITICAL_SUCCESS_STYLE,
                    );
                }
                Outcome::Success => {
                    canvas.write(String::from("success"), color_scheme::SUCCESS_STYLE);
                }
                Outcome::Failure => {
                    canvas.write(String::from("failure"), color_scheme::FAILURE_STYLE);
                }
                Outcome::CriticalFailure => {
                    canvas.write(
                        String::from("critical failure"),
                        color_scheme::CRITICAL_FAILURE_STYLE,
                    );
                }
            }

            if result.damage > 0 {
                canvas.write(String::from(", "), color_scheme::TEXT_STYLE);
                canvas.write(
                    format!("{}", result.damage),
                    color_scheme::TEXT_HIGHLIGHT_STYLE,
                );
                canvas.write(String::from(" damage"), color_scheme::TEXT_STYLE);
            }
        }

        canvas
    }
}
