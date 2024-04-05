use crate::terminal::styled_char::{Style, StyledChar};

pub struct Canvas {
    width: usize,
    height: usize,
    content: Vec<Vec<StyledChar>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            content: vec![vec![StyledChar::new_default(); width]; height],
        }
    }

    pub fn write(&mut self, row: usize, column: usize, content: String, style: Style) {
        for (i, c) in content.chars().enumerate() {
            self.content[row][column + i] = StyledChar::new(c, style);
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
