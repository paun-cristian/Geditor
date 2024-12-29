use std::fs;
use super::view::View;
pub struct File {
    pub modified: bool,
    pub saved: bool,
}

impl File {
    pub fn default() -> Self {
        Self { modified: false, saved: false }
    }
    // pub fn save(&mut self, buffer: Vec<String>) {
    //     write(buffer);

    //     self.saved = true;
    //     self.modified = false;
    // }
}