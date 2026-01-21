use crate::editor::Editor;

pub(crate) const VERSION: f64 = 0.1;
pub(crate) const HEADER_ROWS: usize = 1;
pub(crate) const FOOTER_ROWS: usize = 2;
pub(crate) const TAB: usize = 8;

impl Editor {
    pub(crate) const KEYBINDS: [&'static str; 3] = [
        "<Ctrl-q>: Exit",
        "<Ctrl-s>: Save",
        "<Ctrl-o>: Save as",
    ];
}