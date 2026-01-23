use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::editor::{Editor, state::State};

impl Editor {
    pub fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match self.state {
            State::Editing => self.handle_editor(event),
            State::Quit => self.handle_quit(event),
            State::SaveAs => self.handle_save_as(event),
        }
    }

    pub fn handle_quit(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match event {
            // Save + Quit
            KeyEvent { 
                code: KeyCode::Char('y'), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.save()?;
                Ok(false)
            },
            // Quit
            KeyEvent { 
                code: KeyCode::Char('n'), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => Ok(false),

            // Cancel
            KeyEvent { 
                code: KeyCode::Char('c'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.state = State::Editing;
                Ok(true)
            },
            _ => Ok(true),
        }
    }

    pub fn handle_save_as(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match event {
            // Cancel
            KeyEvent { 
                code: KeyCode::Char('c'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.input = String::new();
                self.state = State::Editing;
                Ok(true)
            },

            // Typing
            KeyEvent { 
                code: KeyCode::Char(c), 
                modifiers: upper @ (KeyModifiers::NONE | KeyModifiers::SHIFT), 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.input.push(
                    if upper == KeyModifiers::SHIFT { 
                        c.to_ascii_uppercase() 
                    } else { 
                        c 
                    }
                );
                Ok(true)
            },

            // Delete
            KeyEvent { 
                code: KeyCode::Backspace, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.input.pop();
                Ok(true)
            },

            // Save
            KeyEvent { 
                code: KeyCode::Enter, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.file = Some(PathBuf::from(&self.input));
                self.save()?;
                self.dirty = false;
                self.state = State::Editing;
                Ok(true)
            },
            _ => Ok(true)
        }
    }

    pub fn handle_editor(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match event {
            // Save
            KeyEvent { 
                code: KeyCode::Char('s'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                if self.file.is_none() {
                    self.state = State::SaveAs;
                    return Ok(true)
                }

                self.save()?;
                self.dirty = false;
            },
            
            // Save As
            KeyEvent { 
                code: KeyCode::Char('o'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                self.state = State::SaveAs;
            },

            // Quit
            KeyEvent { 
                code: KeyCode::Char('q'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                if self.dirty {
                    self.state = State::Quit;
                    return Ok(true)
                } else {
                    return Ok(false)
                }
            },

            // Cursor control
            KeyEvent { 
                code: dir @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down), 
                modifiers: ctrl @ (KeyModifiers::NONE | KeyModifiers::CONTROL), 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.cursor_control(
                dir,
                if ctrl == KeyModifiers::CONTROL { true } else { false },
            ),
            // Home
            KeyEvent { 
                code:  KeyCode::Home, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.home(),
            
            // End
            KeyEvent { 
                code:  KeyCode::End, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.end(),

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
            } => self.delete_char(),

            // CTRL + Delete ( = CTRL + H? )
            KeyEvent { 
                code: KeyCode::Char('h'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.delete_chunk(),

            // Enter
            KeyEvent { 
                code: KeyCode::Enter, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.insert_newline(),

            // Help toggle
            KeyEvent { 
                code: KeyCode::Char('h'), 
                modifiers: KeyModifiers::ALT, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.show_help = !self.show_help,
            _ => (),
        }

        Ok(true)

    }

    fn insert_char(&mut self, ch: char) {
        self.content.insert_char(self.curs, ch);
        self.cursor_control(KeyCode::Right, false);

        self.dirty = true;
    }
    fn delete_char(&mut self) {
        if self.curs == 0 {
            return;
        }

        self.dirty = true;

        if self.curs == self.line_start() {
            let start = self.line_start();
            
            let data = self.content.line(self.line_idx()).to_string();
            self.content.remove(start-1..start-1 + data.len());
            self.render_y -= 1;
            self.end();
            self.content.insert(self.curs, &data.trim());
            return;
        }

        self.content.remove(self.curs-1..self.curs);
        self.curs -= 1;
        self.render_x -= 1;
    }
    fn delete_chunk(&mut self) {
        if self.curs == 0 {
            return;
        }
        self.dirty = true;

        if self.curs == self.line_start() {
            let start = self.line_start();
            
            let data = self.content.line(self.line_idx()).to_string();
            self.content.remove(start-1..start-1 + data.len());
            self.render_y -= 1;
            self.end();
            self.content.insert(self.curs, &data.trim());
            return;
        }

        let start = self.chunk_start();
        self.content.remove(start..self.curs);
        self.move_cursor(start);
    }
    fn insert_newline(&mut self){
        self.dirty = true;
        self.insert_char('\n');
        self.home();
    }
}