use crate::terminal::canvas::Canvas;
use crate::terminal::styled_char::Style;

pub fn add_rounded_borders(original: &Canvas, style: Style) -> Canvas {
    let width = original.get_width() + 2;
    let height = original.get_height() + 2;

    let mut canvas = Canvas::new_transparent(width, height);

    canvas.set_cursor_position(0, 0);
    canvas.write(String::from("\u{256d}"), style);

    canvas.set_cursor_position(0, width - 1);
    canvas.write(String::from("\u{256e}"), style);

    canvas.set_cursor_position(height - 1, 0);
    canvas.write(String::from("\u{2570}"), style);

    canvas.set_cursor_position(height - 1, width - 1);
    canvas.write(String::from("\u{256f}"), style);

    for col in 1..width - 1 {
        canvas.set_cursor_position(0, col);
        canvas.write(String::from("\u{2500}"), style);

        canvas.set_cursor_position(height - 1, col);
        canvas.write(String::from("\u{2500}"), style);
    }

    for row in 1..height - 1 {
        canvas.set_cursor_position(row, 0);
        canvas.write(String::from("\u{2502}"), style);

        canvas.set_cursor_position(row, width - 1);
        canvas.write(String::from("\u{2502}"), style);
    }

    canvas.paste(original, 1, 1);

    canvas
}
