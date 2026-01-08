use std::{fs, path::PathBuf};

use crossterm::terminal;

use crate::{editor::{Editor, consts::HEADER_ROWS}, row::Row};

impl Editor {
    pub fn new() -> std::io::Result<Self> {
        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;
        terminal::enable_raw_mode()?;
        Ok( Self { 
            file: None,
            dirty: false,
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
    pub fn from_file(file: PathBuf) -> std::io::Result<Self> {
        let (col, row) = terminal::size().map(|(c, r)| (c, r))?;
        terminal::enable_raw_mode()?;
        Ok( Self { 
            row_contents: fs::read_to_string(&file)?.lines().map(|l| Row::new(l.into())).collect(),
            file: Some(file),
            dirty: false,
            col: col.into(), 
            row: row.into(), 
            contents: String::new(), 
            curs_x: 0, 
            curs_y: 0,
            render_x: 0,
            render_y: HEADER_ROWS,
            row_offset: 0,
            col_offset: 0,
        })
    }
}