use std::{fs, io::{ErrorKind, Write, stdout}, path::PathBuf};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, terminal};

use crate::row::Row;

pub(crate) mod consts;
mod render;
use consts::*;

pub mod constructors;

pub struct Editor {
    file: Option<PathBuf>,
    dirty: bool,

    col: usize,
    row: usize,

    contents: String,
    row_contents: Vec<Row>,

    curs_x: usize,
    curs_y: usize,

    render_x: usize,
    render_y: usize,

    row_offset: usize,
    col_offset: usize,
}
impl Editor {
    pub fn run(&mut self) -> std::io::Result<bool> {
        self.render_screen()?;
        match event::read()? {
            Event::FocusGained => Ok(true),
            Event::FocusLost => Ok(true),
            Event::Key(key_event) => {
                self.handle_key(key_event)
            },
            Event::Mouse(_) => Ok(true),
            Event::Paste(_) => Ok(true),
            Event::Resize(c, r) => {
                self.col = c.into();
                self.row = r.into();
                Ok(true)
            },
        }
    }
    pub fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match event {
            // Quit
            KeyEvent { 
                code: KeyCode::Char('q'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => return self.handle_quit(),

            // Cursor control
            KeyEvent { 
                code: dir @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down | KeyCode::Home | KeyCode::End), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.move_cursor(dir)?,

            // Typing
            KeyEvent { 
                code: code @ (KeyCode::Char(..) | KeyCode::Tab), 
                modifiers: cap @ (KeyModifiers::NONE | KeyModifiers::SHIFT), 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE, 
            } => self.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(ch) => {
                    match cap {
                        KeyModifiers::NONE => ch,
                        KeyModifiers::SHIFT => ch.to_ascii_uppercase(),
                        _ => unreachable!()
                    }
                },
                _ => unreachable!(),
            }),
            // Delete
            KeyEvent { 
                code: KeyCode::Backspace, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.delete_char()?,

            // Enter
            KeyEvent { 
                code: KeyCode::Enter, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.insert_newline()?,

            // Save
            KeyEvent { 
                code: KeyCode::Char('s'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.save()?;
                self.dirty = false;
            },
            _ => (),
        }

        Ok(true)

    }
    fn handle_quit(&mut self) -> std::io::Result<bool> {
        if !self.dirty {
            Ok(false)
        } else {
            Ok(true)
        }


    }
    fn move_cursor(&mut self, dir: KeyCode) -> std::io::Result<()> {
        match dir {
            KeyCode::Up => {
                if self.curs_y != 0 {
                    self.curs_y -= 1;
                    
                    if self.render_y == HEADER_ROWS {
                        // Shift screen
                        self.row_offset -= 1;
                    }

                }
            },
            KeyCode::Down => {
                if self.curs_y != self.row_contents.len() {
                    self.curs_y += 1;

                    if self.render_y == self.row - FOOTER_ROWS - 1 {
                        // Shift screen
                        self.row_offset += 1;
                    }
                }
            },
            KeyCode::Left => {
                if self.curs_x != 0 {
                    self.curs_x -= 1;

                    if self.render_x == 0 {
                        // Shift screen
                        self.col_offset -= 1;
                    }
                } else {
                    // Special case
                    // Jump to end of previous line

                    if self.curs_y != 0 {
                        self.curs_y -= 1;
                        if self.render_y == HEADER_ROWS {
                            self.row_offset -= 1;
                        }

                        // End function
                        let row = &self.row_contents[self.curs_y];
                        let chars = row.rlen();

                        if chars > self.col {
                            // Shift screen
                            self.col_offset = chars - (self.col - 1);
                        }

                        self.curs_x = row.len();
                    }
                }
            },
            KeyCode::Right => {
                if self.curs_y != self.row_contents.len() {
                    if self.curs_x != self.row_contents[self.curs_y].len() {
                        self.curs_x += 1;
                    } else {
                        // Special case
                        // Jump to start of next line
    
                        self.curs_y += 1;
                        self.curs_x = 0;
                        self.col_offset = 0;
                        
                        if self.render_y == self.row - FOOTER_ROWS - 1 {
                            // Shift screen
                            self.row_offset += 1;
                        }
                    }
                }

            },
            KeyCode::Home => {
                // Go to start of line
                self.col_offset = 0;
                self.curs_x = 0;
            },
            KeyCode::End => {
                if self.curs_y != self.row_contents.len() {
                    // Go to end of line
                    let row = &self.row_contents[self.curs_y];
                    let chars = row.rlen();
                    if chars > self.col {
                        // Multi screen
                        self.col_offset = chars - (self.col - 1);
                    } else {
                        // Single screen
                        self.col_offset = 0;
                    }
                    self.curs_x = row.len();
                }
            },
            _ => unreachable!(),
        }

        // Extra buffer line
        if self.curs_y != self.row_contents.len() {
            // Force cursor to end of line if out of bounds ( OOB )
            let row = &self.row_contents[self.curs_y];
            let chars = row.len();
            if self.curs_x > chars {
                if row.rlen().saturating_sub(self.col) > self.col_offset || self.col_offset >= row.rlen() {
                    // Shift screen ( with padding )
                    self.col_offset = row.rlen().saturating_sub(3);
                }
                self.curs_x = chars;
            }
    
            self.render_x = self.row_contents[self.curs_y].content[..self.curs_x]
                .chars()
                .fold(0, |render_x, c| {
                    if c == '\t' {
                        render_x + 8
                    } else {
                        render_x + 1
                    }
                }) - self.col_offset;
    
        } else {
            self.curs_x = 0;
            self.render_x = 0;
        }
        
        self.render_y = (self.curs_y - self.row_offset) + HEADER_ROWS;


        Ok(())
    }
    fn insert_char(&mut self, ch: char) {
        if self.curs_y == self.row_contents.len() {
            self.row_contents.push(Row::default());
        }

        self.row_contents[self.curs_y].insert_char(self.curs_x, ch);
        self.curs_x += 1;
        self.render_x = self.row_contents[self.curs_y].content[..self.curs_x]
            .chars()
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + 8
                } else {
                    render_x + 1
                }
            }) - self.col_offset;

        self.dirty = true;
    }
    fn delete_char(&mut self) -> std::io::Result<()>{
        if self.curs_y == 0 && self.curs_x == 0 {
            return Ok(())
        }

        if self.curs_y == self.row_contents.len() {
            self.move_cursor(KeyCode::Left)?;
        } else if self.curs_x == 0 {
            let data = self.row_contents.remove(self.curs_y);
            self.move_cursor(KeyCode::Left)?;
            self.row_contents[self.curs_y].content.push_str(&data.content);
            self.row_contents[self.curs_y].re_render();

            self.dirty = true;
        } else {
            self.row_contents[self.curs_y].delete_char(self.curs_x - 1);
            self.curs_x -= 1;
            self.render_x = self.row_contents[self.curs_y].content[..self.curs_x]
            .chars()
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + 8
                } else {
                    render_x + 1
                }
            }) - self.col_offset;

            self.dirty = true;
        }

        Ok(())
    }
    fn insert_newline(&mut self) -> std::io::Result<()>{
        if self.curs_x == 0 {
            self.row_contents.insert(self.curs_y, Row::default());
        } else {
            let data = self.row_contents[self.curs_y].content.split_off(self.curs_x);
            self.row_contents[self.curs_y].re_render();

            self.row_contents.insert(self.curs_y + 1, Row::new(data));
        }
        self.curs_x = 0;
        self.render_x = 0;
        self.curs_y += 1;
        self.render_y = (self.curs_y - self.row_offset) + HEADER_ROWS;

        self.dirty = true;
        Ok(())
    }
    fn save(&self) -> std::io::Result<()> {
        match &self.file {
            None => Err(std::io::Error::new(ErrorKind::Other, "no file name specified")),
            Some(name) => {
                let mut file = fs::OpenOptions::new().write(true).create(true).open(name)?;
                let contents: String = self
                    .row_contents
                    .iter()
                    .map(|r| r.content.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n");
                file.set_len(contents.len() as u64)?;
                file.write_all(contents.as_bytes())
            }
        }
    }
}
impl std::io::Write for Editor {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.contents.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(std::io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        write!(stdout(), "{}", self.contents)?;
        stdout().flush()?;
        self.contents.clear();

        Ok(())
    }
}
impl Drop for Editor {
    fn drop(&mut self){
        terminal::disable_raw_mode().expect("Couldn't disable raw mode");
        Self::clear_screen().expect("Failed to clear screen");
    }
}