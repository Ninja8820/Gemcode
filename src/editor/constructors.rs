use std::{fs, io::stdout, path::PathBuf};

use crossterm::{execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ropey::Rope;

use crate::editor::{Editor, consts::HEADER_ROWS};

impl Editor {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;

        Ok( Self { 
            dirty: false,

            col: col.into(), 
            row: row.into(), 
            
            curs: 0, 

            render_x: 0,
            render_y: HEADER_ROWS,

            row_offset: 0,
            col_offset: 0,

            content: Rope::new(),
            file: None,
        })
    }
    pub fn from_file(file: PathBuf) -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;

        Ok( Self { 
            dirty: false,

            col: col.into(), 
            row: row.into(), 

            curs: 0,

            render_x: 0,
            render_y: HEADER_ROWS,

            row_offset: 0,
            col_offset: 0,

            content: Rope::from_str(&fs::read_to_string(&file)?),
            file: Some(file),
        })
    }
}

impl Drop for Editor {
    fn drop(&mut self){
        terminal::disable_raw_mode().expect("Couldn't disable raw mode");
        execute!(stdout(), LeaveAlternateScreen).expect("Failed to return to original screen");
    }
}