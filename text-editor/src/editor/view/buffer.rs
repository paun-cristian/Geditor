pub struct Buffer {
    pub lines: Vec<String>
}

impl Default for Buffer {
    fn default() -> Self {
        let mut lines: Vec<String> = Vec::new();
        let size = super::Terminal::get_terminal_size().unwrap();
        lines.push("Hello, world!".to_string());
        for _ in 1..size.height {
            lines.push("~".to_string());
        }
        Self {lines}
    }
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}