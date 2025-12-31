use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, queue, style::{self, Color}, terminal::{self, ClearType}};
use std::{cmp, fs, io::{Write, stdout}, path::PathBuf};
use clap::Parser;

const VERSION: f64 = 0.1;
const HEADER_ROWS: usize = 1;
const FOOTER_ROWS: usize = 2;
const TAB: usize = 8;
struct Cleaner;
impl Drop for Cleaner {
    fn drop(&mut self){
        terminal::disable_raw_mode().expect("Couldn't disable raw mode");
        Editor::clear_screen().expect("Failed to clear screen");
    }
}
struct Row {
    content: String,
    render: String,
}
impl Row {
    fn new(content: String) -> Self {
        
        Self {
            render: content.replace("\t", &" ".repeat(TAB)),
            content,
        }
    }
    pub fn rlen(&self) -> usize {
        self.render.len()
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }
}
struct Editor {
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
            } => Ok(false),

            // Cursor control
            KeyEvent { 
                code: dir @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down | KeyCode::Home | KeyCode::End), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => self.move_cursor(dir),
            _ => Ok(true),
        }

    }
    fn move_cursor(&mut self, dir: KeyCode) -> std::io::Result<bool> {
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
                if self.curs_y != self.row_contents.len() - 1 {
                    self.curs_y += 1;

                    if self.render_y == self.row - FOOTER_ROWS - HEADER_ROWS {
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
                if self.curs_x != self.row_contents[self.curs_y].len() - 1 {
                    self.curs_x += 1;
                } else {
                    //Special case
                    // Jump to start of next line

                    if self.curs_y != self.row_contents.len() - 1 {
                        self.curs_y += 1;
                        self.curs_x = 0;
                        self.col_offset = 0;
    
                        if self.render_y == self.row - FOOTER_ROWS - HEADER_ROWS {
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
            },
            _ => unreachable!(),
        }

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

        self.render_y = (self.curs_y - self.row_offset) + HEADER_ROWS;


        Ok(true)
    }
    fn clear_screen() -> std::io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        Ok(())
    }
    
    fn render_screen(&mut self) -> std::io::Result<()> {
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
        let mut header = format!("Gemcode --- Version {}", VERSION);
        if header.len() > self.col {
            header.truncate(self.col)
        }
        self.contents.push_str(&header);
        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
        self.contents.push_str("\r\n");

        Ok(())
    }
    fn body(&mut self) -> std::io::Result<()> {
        for i in HEADER_ROWS..self.row-FOOTER_ROWS {
            if i <= self.row_contents.len() {
                let index = self.row_offset + (i - HEADER_ROWS);
                let row = &self.row_contents[index];
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
    
    fn new() -> std::io::Result<Self> {
        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;
        Ok( Self { 
            col: col.into(), 
            row: row.into(), 
            contents: String::new(), 
            row_contents: Vec::new(),
            curs_x: 0, 
            curs_y: 0,
            render_x: 0,
            render_y: HEADER_ROWS,
            row_offset: 0,
            col_offset: 0,
        })
    }
    fn from_file(file: &PathBuf) -> std::io::Result<Self> {
        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;
        Ok( Self { 
            col: col.into(), 
            row: row.into(), 
            contents: String::new(), 
            row_contents: fs::read_to_string(file)?.lines().map(|l| Row::new(l.into())).collect(),
            curs_x: 0, 
            curs_y: 0,
            render_x: 0,
            render_y: HEADER_ROWS,
            row_offset: 0,
            col_offset: 0,
        })
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
#[derive(Parser, Debug)]
struct Args {
    // File to edit
    file: Option<PathBuf>,
}
fn main() -> std::io::Result<()>{
    let args = Args::parse();

    let _cleaner = Cleaner;
    terminal::enable_raw_mode()?;

    let mut ed = if let Some(f) = args.file {
        Editor::from_file(&f)?
    } else {
        Editor::new()?
    };

    while ed.run()? {

    }


    Ok(())
}