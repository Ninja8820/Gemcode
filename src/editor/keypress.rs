use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::{editor::Editor};

impl Editor {
    pub fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match event {
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

            // Quit
            KeyEvent { 
                code: KeyCode::Char('q'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => return self.handle_quit(),            

            // Cursor control
            KeyEvent { 
                code: dir @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.move_cursor(dir),
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
            // KeyEvent { 
            //     code: KeyCode::Char('h'), 
            //     modifiers: KeyModifiers::CONTROL, 
            //     kind: KeyEventKind::Press, 
            //     state: KeyEventState::NONE 
            // } => self.delete(true)?,

            // Enter
            KeyEvent { 
                code: KeyCode::Enter, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.insert_newline(),
            _ => (),
        }

        Ok(true)

    }
    fn handle_quit(&mut self) -> std::io::Result<bool> {
        if !self.dirty {
            Ok(false)
        } else {
            Ok(false)
        }
    }

    fn insert_char(&mut self, ch: char) {
        self.content.insert_char(self.curs, ch);
        self.move_cursor(KeyCode::Right);

        self.dirty = true;
    }
    fn delete_char(&mut self) {
        self.dirty = true;

        if self.curs != 0 && self.curs == self.line_start() {
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
    fn insert_newline(&mut self){
        self.dirty = true;
        self.insert_char('\n');
        self.home();
    }
}