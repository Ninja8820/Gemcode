use crossterm::event::KeyCode;

use crate::editor::{Editor, consts::{FOOTER_ROWS, HEADER_ROWS}};

impl Editor {
    pub fn home(&mut self) {
        // Go to start of line
        self.curs = self.line_start();

        self.col_offset = 0;
        self.render_x = 0;
    }
    pub fn end(&mut self) {
        self.curs = self.line_end();

        let chars = self.line_len();
        if chars > self.col + self.col_offset {
            // Off-screen ( Right )
            self.col_offset = chars - self.col + 1;
            self.render_x = self.col - 2;

        } else if chars < self.col_offset {
            // Off-screen ( Left )
            self.col_offset = chars.saturating_sub(4);
            self.render_x = if chars < 4 { chars - 1 } else { 3 };

        } else {
            // On-screen
            self.render_x = chars - self.col_offset;
        }
    }
    pub fn move_cursor(&mut self, dir: KeyCode) {
        match dir {
            KeyCode::Up => {
                if self.line_idx() != 0 {
                    if self.render_y == HEADER_ROWS {
                        // Scroll screen
                        self.row_offset -= 1;
                    } else {
                        self.render_y -= 1;
                    }
                    if self.line_len() < self.render_x + self.col_offset {
                        // Force to end
                        self.end();

                    } else {
                        // Slot into next line
                        self.curs = self.line_start() + self.col_offset + self.render_x;
                    }
                }
            },
            KeyCode::Down => {
                if self.line_idx() != self.content.len_lines() - 1 {
                    if self.render_y == self.row - FOOTER_ROWS - 1 {
                        // Scroll screen
                        self.row_offset += 1;
                    } else {
                        self.render_y += 1;
                    }
                    if self.line_len() < self.render_x + self.col_offset {
                        // Force to end
                        self.end();

                    } else {
                        // Slot into next line
                        self.curs = self.line_start() + self.col_offset + self.render_x;
                    }
                }
            },
            KeyCode::Left => {
                if self.curs != 0 {
                    if self.curs != self.line_start() {
                        if self.render_x == 0 {
                            // Off-screen
                            self.col_offset -= 1;
                        } else {
                            // On-screen
                            self.render_x -= 1;
                        }

                        // Move left by the byte size of the previous character
                        self.curs -= self.content.slice(0..self.curs)
                            .to_string()
                            .chars()
                            .last()
                            .unwrap_or('\0')
                            .len_utf8();
                    } else {
                        // -- Special case --
                        // Jump to end of previous line
                        self.render_y -= 1; 
                        self.end();
                    };
                }
            },
            KeyCode::Right => {
                if self.curs != self.content.len_chars() {
                    if self.curs != self.line_end() {
                        if self.render_x == self.col - 1 {
                            // Off-screen
                            self.col_offset += 1;
                        } else {
                            // On-screen
                            self.render_x += 1;
                        }

                        // Move right by the byte size of the next character
                        self.curs += self.content.slice(self.curs..)
                            .to_string()
                            .chars()
                            .next()
                            .unwrap_or('\0')
                            .len_utf8();

                    } else {
                        // -- Special case --
                        // Jump to start of next line
                        self.render_y += 1; 
                        self.home();
                    }

                }
            },
            _ => unreachable!(),
        }
    }
}