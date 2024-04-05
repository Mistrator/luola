use crate::terminal::canvas::Canvas;
use std::mem;

mod ansi_sequences;
mod canvas;
pub mod color;
pub mod styled_char;

pub struct Terminal {
    width: usize,
    height: usize,

    current_frame: Canvas,
    pub next_frame: Canvas,
}

impl Terminal {
    pub fn init(width: usize, height: usize) -> Self {
        println!("{}", ansi_sequences::use_alternate_screen_buffer());
        println!("{}", ansi_sequences::clear_screen());

        Self {
            width,
            height,
            current_frame: Canvas::new(width, height),
            next_frame: Canvas::new(width, height),
        }
    }

    pub fn render_next(&mut self) {
        let current_rendered = self.current_frame.render();
        let next_rendered = self.next_frame.render();

        // todo: render only characters that differ between current and next frames
        println!("{}", ansi_sequences::set_cursor_position(1, 1));
        for line in next_rendered {
            for c in line {
                print!("{}", c);
            }
            println!();
        }

        mem::swap(&mut self.current_frame, &mut self.next_frame);
        self.next_frame = Canvas::new(self.width, self.height);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        println!("{}", ansi_sequences::clear_screen());
        println!("{}", ansi_sequences::use_main_screen_buffer());
    }
}
