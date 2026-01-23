use crossterm::event::KeyCode;

use crate::editor::{Editor};

impl Editor {
    pub fn home(&mut self) {
        // Go to start of line
        self.curs = self.line_start();

        self.col_offset = 0;
        self.render_x = 0;
    }
    pub fn end(&mut self) {
        // self.move_cursor(self.line_end());
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
    pub fn move_cursor(&mut self, char_idx: usize) {
        self.curs = char_idx;

        let line = self.content.char_to_line(char_idx);
        let chars = char_idx - self.content.line_to_char(line);

        // Scroll screen ( Horizontal )
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

        // Scroll screen ( Vertical )
        let curr = self.line_idx();
        if curr != line {
            if line < self.row_offset {
                // Off-screen ( Up )
                self.row_offset = line;
                self.render_y = Self::HEADER_ROWS;

            } else if line > self.row_offset + self.row - 1 {
                // Off-screen ( Down )
                self.row_offset = line - self.row;
                self.render_y = self.row - self.layout().footer;

            } else {
                // On-screen
                self.render_y = line - self.row_offset + Self::HEADER_ROWS
            }
        }
    }
    pub fn cursor_control(&mut self, dir: KeyCode, ctrl: bool) {
        match dir {
            KeyCode::Up => {
                if ctrl {
                    if self.row_offset != 0 && self.render_y != self.row - 1 - self.layout().footer {
                        self.row_offset = self.row_offset.saturating_sub(1);
                        self.render_y += 1;
                    }
                } else {
                    if self.line_idx() != 0 {
                        if self.render_y == Self::HEADER_ROWS {
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
                }
            },
            KeyCode::Down => {
                if ctrl {
                    if self.row_offset + self.row != self.content.len_lines() - 1 && self.render_y != Self::HEADER_ROWS {
                        self.row_offset = self.row_offset.saturating_add(1);
                        self.render_y -= 1;
                    }
                } else {
                    if self.line_idx() != self.content.len_lines() - 1 {
                        if self.render_y == self.row - 1 - self.layout().footer {
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
                }
            },
            KeyCode::Left => {
                if self.curs != 0 {
                    if self.curs != self.line_start() {
                        if ctrl {
                            self.move_cursor(self.chunk_start());

                        } else {
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
                        }
                    } else {
                        // -- Special case --
                        // Jump to end of previous line
                        if self.render_y == Self::HEADER_ROWS {
                            self.row_offset -= 1;
                        } else {
                            self.render_y -= 1; 
                        }
                        self.end();
                    };
                }
            },
            KeyCode::Right => {
                if self.curs != self.content.len_chars() {
                    if self.curs != self.line_end() {
                        if ctrl {
                            self.move_cursor(self.chunk_end());
                        } else {
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
                        }

                    } else {
                        // -- Special case --
                        // Jump to start of next line
                        if self.render_y == self.row - 1 - self.layout().footer {
                            self.row_offset += 1;
                        } else {
                            self.render_y += 1; 
                        }
                        self.home();
                    }

                }
            },
            _ => unreachable!(),
        }
    }
}