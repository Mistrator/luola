use crate::terminal::color::Color;
use crate::terminal::styled_char::{Style, StyledChar};

pub struct Canvas {
    width: usize,
    height: usize,
    content: Vec<Vec<StyledChar>>,
    cursor_row: usize,
    cursor_column: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            content: vec![vec![StyledChar::new_default(); width]; height],
            cursor_row: 0,
            cursor_column: 0,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_cursor_position(&mut self, row: usize, column: usize) {
        self.cursor_row = row;
        self.cursor_column = column;
    }

    pub fn write(&mut self, content: String, style: Style) {
        for c in content.chars() {
            self.content[self.cursor_row][self.cursor_column] = StyledChar::new(c, style);

            self.cursor_column += 1;
            if self.cursor_column >= self.width {
                self.cursor_row += 1;
                self.cursor_column = 0;
            }
        }
    }

    pub fn paste(&mut self, other: &Canvas, row: usize, column: usize) {
        assert!(
            row + other.get_height() <= self.height,
            "pasted canvas not contained in this canvas"
        );
        assert!(
            column + other.get_width() <= self.width,
            "pasted canvas not contained in this canvas"
        );

        for (i, content_row) in other.content.iter().enumerate() {
            for (j, c) in content_row.iter().enumerate() {
                let current_c = self.content[row + i][column + j];
                let mut pasted_c = *c;

                if pasted_c.style.foreground_color == Color::Transparent {
                    pasted_c.style.foreground_color = current_c.style.foreground_color;
                }
                if pasted_c.style.background_color == Color::Transparent {
                    pasted_c.style.background_color = current_c.style.background_color;
                }

                self.content[row + i][column + j] = pasted_c;
            }
        }
    }

    pub fn render(&self) -> Vec<Vec<String>> {
        let mut rendered: Vec<Vec<String>> = Vec::new();

        for row in &self.content {
            let mut rendered_row: Vec<String> = Vec::new();

            for c in row {
                let rendered_char = c.render();
                rendered_row.push(rendered_char);
            }

            rendered.push(rendered_row);
        }

        rendered
    }
}
