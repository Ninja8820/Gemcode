use crate::editor::Editor;

impl Editor {

    pub fn line_idx(&self) -> usize {
        self.row_offset + self.render_y - Self::HEADER_ROWS
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
    pub fn chunk_start(&self) -> usize {
        let ws = self.content.char(self.curs - 1).is_whitespace();

        let mut at = Some(self.curs - 1);
        while let Some(p) = at && self.content.char(p).is_whitespace() == ws {            
            at = p.checked_sub(1);
        }

        match at {
            Some(p) => p + 1,
            None => 0,
        }
    }
    pub fn chunk_end(&self) -> usize {
        let ws = self.content.char(self.curs).is_whitespace();
        let max = self.content.len_chars();

        let mut at = Some(self.curs);
        while let Some(p) = at && p < max && self.content.char(p).is_whitespace() == ws {            
            at = p.checked_add(1);
        }

        match at {
            Some(p) => p,
            None => self.content.len_chars() - 1,
        }
    }
}