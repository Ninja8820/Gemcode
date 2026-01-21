use crate::editor::{Editor, consts::HEADER_ROWS};

impl Editor {

    pub fn line_idx(&self) -> usize {
        self.row_offset + self.render_y - HEADER_ROWS
    }
    pub fn line_start(&self) -> usize {
        self.content.line_to_char(self.line_idx())
    }
    pub fn line_end(&self) -> usize {
        self.line_start() + self.line_len()
    }
    pub fn line_len(&self) -> usize {
        let line_idx = self.line_idx();
        let line = self.content.line(line_idx);

        if line_idx != self.content.len_lines() - 1 {
            line.len_chars() - 1
        } else {
            line.len_chars()
        }
    }
}