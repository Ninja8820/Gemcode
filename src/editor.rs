use std::{fs, io::{ErrorKind, Write}, path::PathBuf};
use crossterm::event::{self, Event};
use ropey::Rope;

use crate::editor::state::State;

pub(crate) mod consts;
mod render;
mod helper;
mod keypress;
mod cursor;

pub mod constructors;
pub(crate) mod state;

pub struct Editor {
    state: State,
    input: String,

    file: Option<PathBuf>,
    dirty: bool,

    col: usize,
    row: usize,
    
    content: Rope,

    curs: usize,

    render_x: usize,
    render_y: usize,

    row_offset: usize,
    col_offset: usize,

    show_help: bool,
}
impl Editor {
    pub fn run<W: Write>(&mut self, out: &mut W) -> std::io::Result<bool> {
        self.render_screen(out)?;
        
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
    fn save(&self) -> std::io::Result<()> {
        match &self.file {
            None => Err(std::io::Error::new(ErrorKind::Other, "No file name specified")),
            Some(name) => {
                let file = fs::OpenOptions::new().write(true).create(true).open(name)?;
                self.content.write_to(&file)
            }
        }
    }
}