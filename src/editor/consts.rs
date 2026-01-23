use crate::editor::Editor;


impl Editor {
    pub(crate) const VERSION: f64 = 0.1;
    pub(crate) const HEADER_ROWS: usize = 1;
    // pub(crate) const TAB: usize = 8;
    
    pub(crate) const KEYBINDS: [&'static str; 4] = [
        "<Ctrl-q>: Exit",
        "<Ctrl-s>: Save",
        "<Ctrl-o>: Save as",
        "<Alt-h>: Toggle help",
    ];
    pub(crate) const MAX_BIND: usize = 20;

    pub(crate) const CONFIRMATION: [&'static str; 3] = [
        "<Y>: Yes",
        "<N>: No",
        "<Ctrl-c>: Cancel",
    ];
    pub(crate) const MAX_CONF: usize = 16;

    pub(crate) const SAVE_AS: [&'static str; 2] = [
        "<Enter>: Save",
        "<Ctrl-c>: Cancel",
    ];
    pub(crate) const MAX_SAVE: usize = 16;
}