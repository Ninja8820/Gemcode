use std::path::PathBuf;
use clap::Parser;

pub mod row;
pub mod editor;
use crate::editor::Editor;

#[derive(Parser, Debug)]
struct Args {
    // File to edit
    file: Option<PathBuf>,
}
fn main() -> std::io::Result<()>{
    let args = Args::parse();

    let mut ed = if let Some(f) = args.file {
        Editor::from_file(f)?
    } else {
        Editor::new()?
    };

    
    while ed.run()? {
        
    }

    Ok(())
}