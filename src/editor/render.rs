use std::{cmp, io::{Write, stdout}};

use crossterm::{cursor, execute, queue, style::Color, terminal::{self, ClearType}};

use crate::editor::{Editor, consts::{FOOTER_ROWS, HEADER_ROWS, VERSION}};

impl Editor {
    pub fn clear_screen() -> std::io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        Ok(())
    }
    
    pub(crate) fn render_screen(&mut self) -> std::io::Result<()> {
        queue!(self, cursor::MoveTo(0, 0), cursor::Hide)?;

        queue!(self, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        self.header()?;
        queue!(self, crossterm::style::ResetColor)?;

        self.body()?;

        queue!(self, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        self.footer()?;
        queue!(self, crossterm::style::ResetColor)?;

        let x = self.render_x as u16;
        let y = self.render_y as u16;
        queue!(self, cursor::MoveTo(x, y), cursor::Show)?;
        
        self.flush()?;

        Ok(())
    }
    fn header(&mut self) -> std::io::Result<()> {
        // Title -- Name -- Lines
        let title = format!("Gemcode --- Version {}", VERSION);
        let lines = format!("{} / {} ", self.curs_y + 1, self.row_contents.len());
        let name = match &self.file {
            Some(f) => {
                f.file_name().expect("Failed to get file name").to_str().expect("File name is invalid").into()
            },
            None => "Buffer",
        };

        let width = self.col / 3;
        if self.col > title.len() + lines.len() + name.len() {
            self.contents.push_str(&format!("{:<width$}{:^width$}{:>width$}", title, &name, lines));        
        }

        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
        self.contents.push_str("\r\n");

        Ok(())
    }
    fn body(&mut self) -> std::io::Result<()> {
        for mut i in 0..self.row - FOOTER_ROWS - HEADER_ROWS {
            i += self.row_offset;
            if i < self.row_contents.len() {
                let row = &self.row_contents[i];
                let len = cmp::min(row.rlen().saturating_sub(self.col_offset), self.col);
                let start = if len == 0 { 0 } else { self.col_offset };
                let text = &row.render[start..start + len];
                self.contents.push_str(&text);
            }

            queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
            self.contents.push_str("\r\n");
        }

        Ok(())
    }
    fn footer(&mut self) -> std::io::Result<()> {
        self.contents.push_str("<Ctrl-q>: Exit");
        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
        self.contents.push_str("\r\n");
        self.contents.push_str("<Ctrl-s>: Save");
        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;

        Ok(())
    }
}