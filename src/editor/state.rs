#[derive(Debug, PartialEq)]
pub enum State {
    Editing,
    Quit,
    SaveAs,
}