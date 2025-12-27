use std::io::{Write, stdout};
use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, queue, style::Color, terminal::{self, ClearType}};

const VERSION: f64 = 0.1;
const HEADER_ROWS: usize = 1;
const FOOTER_ROWS: usize = 2;
struct Cleaner;
impl Drop for Cleaner {
    fn drop(&mut self){
        terminal::disable_raw_mode().expect("Couldn't disable raw mode");
        Editor::clear_screen().expect("Failed to clear screen");
    }
}
struct Editor {
    col: usize,
    row: usize,

    contents: String,

    curs_x: usize,
    curs_y: usize,
}
impl Editor {
    pub fn run(&mut self) -> std::io::Result<bool> {
        self.render_screen()?;
        // if event::poll(Duration::from_millis(5000))? {
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
        // } else {
        //     Ok(true)
        // }
    }
    pub fn handle_key(&mut self, event: KeyEvent) -> std::io::Result<bool> {
        match event {
            KeyEvent { 
                code: KeyCode::Char('q'), 
                modifiers: KeyModifiers::CONTROL, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => Ok(false),
            KeyEvent { 
                code: KeyCode::Down, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                if self.curs_y < self.row - FOOTER_ROWS - 1 {
                    self.curs_y += 1;
                }
                Ok(true)
            },
            KeyEvent { 
                code: KeyCode::Up, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                if self.curs_y > HEADER_ROWS {
                    self.curs_y -= 1;
                }
                Ok(true)
            },
            KeyEvent { 
                code: KeyCode::Left, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                if self.curs_x > 0 {
                    self.curs_x -= 1;
                }
                Ok(true)
            },
            KeyEvent { 
                code: KeyCode::Right, 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            } => {
                if self.curs_x < self.col {
                    self.curs_x += 1;
                }
                Ok(true)
            },
            _ => Ok(true),
        }

    }
    fn clear_screen() -> std::io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        Ok(())
    }
    
    fn render_screen(&mut self) -> std::io::Result<()> {
        queue!(self, cursor::MoveTo(0, 0), cursor::Hide)?;
        // Self::clear_screen()?;

        queue!(self, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        self.header()?;
        queue!(self, crossterm::style::ResetColor)?;

        self.body()?;

        queue!(self, crossterm::style::SetBackgroundColor(Color::DarkGrey))?;
        self.footer()?;
        queue!(self, crossterm::style::ResetColor)?;

        let x = self.curs_x as u16;
        let y = self.curs_y as u16;
        queue!(self, cursor::MoveTo(x, y), cursor::Show)?;
        
        self.flush()?;

        Ok(())
    }
    fn header(&mut self) -> std::io::Result<()> {
        let c = self.col;
        let mut header = format!("Gemcode --- Version {}", VERSION);
        if header.len() > c {
            header.truncate(c)
        }
        self.contents.push_str(&header);
        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
        self.contents.push_str("\r\n");

        Ok(())
    }
    fn body(&mut self) -> std::io::Result<()> {
        for _ in HEADER_ROWS..self.row-FOOTER_ROWS {
            queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
            self.contents.push_str("\n\r");
        }

        Ok(())
    }
    fn footer(&mut self) -> std::io::Result<()> {
        
        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;
        self.contents.push_str("\r<Ctrl-q>: Exit\n");
        self.contents.push_str("\r<Ctrl-s>: Save");
        queue!(self, terminal::Clear(ClearType::UntilNewLine))?;

        Ok(())
    }
    
    fn new() -> std::io::Result<Self> {
        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;
        Ok( Self { 
            col: col.into(), 
            row: row.into(), 
            contents: String::new(), 
            curs_x: 0, 
            curs_y: 1 
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
fn main() -> std::io::Result<()>{
    let _cleaner = Cleaner;
    terminal::enable_raw_mode()?;
    let mut ed = Editor::new()?;
    while ed.run()? {

    }

    Ok(())
}