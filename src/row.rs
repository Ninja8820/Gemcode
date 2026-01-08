use crate::editor::consts::TAB;

#[derive(Default, Debug)]
pub(crate) struct Row {
    pub content: String,
    pub render: String,
}
impl Row {
    pub fn new(content: String) -> Self {
        let mut row = Self {
            content,
            render: String::new(),
        };
        row.re_render();
        row
    }
    pub fn re_render(&mut self) {
        self.render = self.content.replace("\t", &" ".repeat(TAB));
    }

    pub fn rlen(&self) -> usize {
        self.render.len()
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn insert_char(&mut self, at: usize, ch: char) {
        self.content.insert(at, ch);
        self.re_render();
    }
    pub fn delete_char(&mut self, at: usize) {
        self.content.remove(at);
        self.re_render();
    }
    
}