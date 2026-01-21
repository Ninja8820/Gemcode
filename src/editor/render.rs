use std::io::Write;

use crossterm::{cursor::{self}, execute, queue, style::Color, terminal::{self, ClearType}};

use crate::editor::{Editor, consts::{FOOTER_ROWS, HEADER_ROWS, VERSION}};

impl Editor {
    pub fn clear_screen<W: Write>(out: &mut W) -> std::io::Result<()> {
        execute!(out, terminal::Clear(ClearType::All))?;
        execute!(out, cursor::MoveTo(0, 0))?;

        Ok(())
    }
    
    pub(crate) fn render_screen<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        queue!(out, cursor::MoveTo(0, 0), cursor::Hide)?;

        queue!(out, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        self.header(out)?;
        queue!(out, crossterm::style::ResetColor)?;

        self.body(out)?;

        queue!(out, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        self.footer(out)?;
        queue!(out, crossterm::style::ResetColor)?;

        let x = self.render_x as u16;
        let y = self.render_y as u16;
        queue!(out, cursor::MoveTo(x, y), cursor::Show)?;
        
        out.flush()?;

        Ok(())
    }
    fn header<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        // Title -- Name -- Lines
        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;

        let title = format!("Gemcode --- Version {}", VERSION);
        let lines = format!("{} / {} ", self.line_idx() + 1, self.content.len_lines());
        let name = "Buffer";

        let width = self.col / 3;
        if self.col > title.len() + lines.len() + name.len() {
            write!(out, "{:<width$}{:^width$}{:>width$}\r\n", title, &name, lines)?;   
        }

        Ok(())
    }
    fn body<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        let lines = self.content.len_lines();

        for screen_row in 0..self.row-HEADER_ROWS-FOOTER_ROWS {
            let file_row = screen_row + self.row_offset;

            // Clear the current line
            queue!(out, terminal::Clear(ClearType::UntilNewLine))?;

            if file_row < lines {
                let row = self.content.line(file_row);

                // Total characters in the row
                let mut len_chars = row.len_chars();

                // Check if the last character is '\n'
                if len_chars > 0 {
                    let last_char = row.char(len_chars - 1);
                    if last_char == '\n' {
                        len_chars -= 1; // exclude newline from display
                    }
                }

                // Determine horizontal scroll range
                let start = self.col_offset.min(len_chars);
                let end = (self.col_offset + self.col).min(len_chars);

                // Slice by char indices and write
                let visible = row.slice(start..end);
                write!(out, "{}", visible.to_string())?;
            }

            // Move to next line
            write!(out, "\r\n")?;
        }

        Ok(())
    }
    fn footer<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        let max_length = Self::KEYBINDS.iter().map(|s| s.len()).max().unwrap_or(0);
        let mid_point = Self::KEYBINDS.len() / 2;

        let first_half = &Self::KEYBINDS[mid_point..];
        let second_half = &Self::KEYBINDS[..mid_point];

        let mut l1 = String::new();
        l1.push('|');
        for s in first_half {
            let padded = format!("{:width$}", s, width = max_length);
            l1.push_str(&format!("   {}   |", padded));
        }

        let mut l2 = String::new();
        l2.push('|');
        for s in second_half {
            let padded = format!("{:width$}", s, width = max_length);
            l2.push_str(&format!("   {}   |", padded));
        }


        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;
        write!(out, "{:<w$}\r\n", l1, w = self.col)?;   
        queue!(out, terminal::Clear(ClearType::UntilNewLine))?;
        write!(out, "{:<w$}", l2, w = self.col)?;   

        Ok(())
    }
}