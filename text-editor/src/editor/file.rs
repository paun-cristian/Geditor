use std::fs;
use super::view::{self, View};
pub struct File {
    pub modified: bool,
    pub saved: bool,
    pub filename: String,
}

impl File {
    pub fn default() -> Self {
        Self { modified: false, saved: false, filename: String::new() }
    }
}