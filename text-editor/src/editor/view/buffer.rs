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
    pub fn reformat_lines(&mut self, width: u16) {
        let mut new_lines = Vec::new();
        for line in &self.lines {
            let mut start = 0;
            while start < line.len() {
                let end = std::cmp::min(start + width as usize, line.len());
                new_lines.push(line[start..end].to_string());
                start = end;
            }
        }
        self.lines = new_lines;
    }

}