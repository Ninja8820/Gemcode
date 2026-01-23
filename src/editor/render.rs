use std::io::Write;
use crossterm::{cursor::{self}, queue, style::Color, terminal::{self, ClearType}};
use crate::editor::{Editor, state::State};

pub struct Layout {
    pub body: usize,
    pub prompt: usize,
    pub footer: usize,
}

impl Editor {
    
    pub(crate) fn render_screen<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        queue!(out, cursor::MoveTo(0, 0), cursor::Hide)?;

        let layout = self.layout();

        self.header(out)?;
        self.body(out, layout.body)?;
        if layout.prompt > 0 {
            match self.state {
                State::Editing => (),
                State::Quit => self.quit(out)?,
                State::SaveAs => self.save_as(out)?,
            }
        }
        if self.show_help || self.state != State::Editing {
            self.footer(out)?;
        }

        self.cursor(out, layout.footer)?;
        out.flush()?;

        Ok(())
    }
    fn header<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        // Title -- Name -- Lines
        queue!(out, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;

        let title = format!("Gemcode --- Version {}", Self::VERSION);
        let lines = format!("{} / {} ", self.line_idx() + 1, self.content.len_lines());
        let name = match &self.file {
            Some(path) => {
                path.file_name().unwrap().to_str().unwrap()
            },
            None => {
                "Buffer"
            },
        };

        let width = self.col / 3;
        if self.col > title.len() + lines.len() + name.len() {
            write!(out, "{:<width$}{:^width$}{:>width$}", title, &name, lines)?;   
        }

        queue!(out, crossterm::style::ResetColor)?;
        Ok(())
    }
    fn body<W: Write>(&mut self, out: &mut W, rows: usize) -> std::io::Result<()> {
        let lines = self.content.len_lines();

        for screen_row in 0..rows {
            let file_row = screen_row + self.row_offset;

            // Move to next line
            write!(out, "\r\n")?;

            // Clear line
            queue!(out, terminal::Clear(ClearType::UntilNewLine))?;

            if file_row < lines {
                let row = self.content.line(file_row);

                let mut len_chars = row.len_chars();

                if len_chars > 0 {
                    let last_char = row.char(len_chars - 1);
                    if last_char == '\n' {
                        len_chars -= 1;
                    }
                }

                let start = self.col_offset.min(len_chars);
                let end = (self.col_offset + self.col).min(len_chars);

                let visible = row.slice(start..end);
                write!(out, "{}", visible)?;
            }

        }

        queue!(out, crossterm::style::ResetColor)?;
        Ok(())
    }
    fn footer<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        let content = match self.state {
            State::Editing => Self::KEYBINDS.to_vec(),
            State::Quit => Self::CONFIRMATION.to_vec(),
            State::SaveAs => Self::SAVE_AS.to_vec(),
        };
        let max = match self.state {
            State::Editing => Self::MAX_BIND,
            State::Quit => Self::MAX_CONF,
            State::SaveAs => Self::MAX_SAVE,
        };
        let mid_point = content.len() / 2;

        let first_half = &content[mid_point..];
        let second_half = &content[..mid_point];

        let mut l1 = String::new();
        l1.push('|');
        for s in first_half {
            let padded = format!("{:width$}", s, width = max);
            l1.push_str(&format!("   {}   |", padded));
        }

        let mut l2 = String::new();
        l2.push('|');
        for s in second_half {
            let padded = format!("{:width$}", s, width = max);
            l2.push_str(&format!("   {}   |", padded));
        }


        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;
        queue!(out, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        write!(out, "\r\n{:<w$}", l1, w = self.col)?;   
        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;
        write!(out, "\r\n{:<w$}", l2, w = self.col)?;   
        
        queue!(out, crossterm::style::ResetColor)?;
        Ok(())
    }
    fn quit<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;
        queue!(out, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        write!(out, "\r\nSave file:")?;   

        queue!(out, crossterm::style::ResetColor)?;
        Ok(())
    }
    fn save_as<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;
        queue!(out, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        write!(out, "\r\nSave file as: {}", self.input)?;   

        queue!(out, crossterm::style::ResetColor)?;
        Ok(())
    }
    fn cursor<W: Write>(&self, out: &mut W, footer: usize) -> std::io::Result<()> {
        match self.state {
            State::Editing => {
                queue!(out, cursor::MoveTo(self.render_x as u16, self.render_y as u16), cursor::Show)?;
            },
            State::Quit => {
                queue!(out, cursor::MoveTo(10, (self.row - footer - 1) as u16), cursor::Show)?;
            },
            State::SaveAs => {
                queue!(out, cursor::MoveTo(14 + self.input.len() as u16, (self.row - footer - 1) as u16), cursor::Show)?;
            }
        }
        

        Ok(())
    }
    pub fn layout(&self) -> Layout {
        let header = Self::HEADER_ROWS;
        let footer = if self.show_help || self.state != State::Editing { 2 } else { 0 };
        let prompt = match self.state {
            State::Editing => 0,
            State::Quit => 1,
            State::SaveAs => 1,
        };


        Layout { 
            prompt, 
            footer, 
            body: self.row - header - footer - prompt, 
        }
    }
}