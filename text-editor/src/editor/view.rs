mod buffer;
use buffer::Buffer;
use super::{terminal, Terminal};

#[derive(Default, Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Clone, Copy)]
pub struct ScrollOffset {
    pub x: usize,
    pub y: usize,
}

pub struct View {
    pub buffer: Buffer,
    pub scroll_offset: ScrollOffset,
}

impl View {
    pub fn default() -> Self {
        Self { buffer: Buffer::default(), scroll_offset: ScrollOffset { x: 0, y: 0 } }
    }
    pub fn render_buffer(&self) -> Result<(), std::io::Error> {
        let terminal::Size { height, width } = Terminal::get_terminal_size()?;
        
        // Determine the range of buffer lines to render based on scroll_offset
        let start_line = self.scroll_offset.y;
        let end_line = core::cmp::min(start_line + height as usize, self.buffer.lines.len());

        for current_row in 0..self.buffer.lines.len() as usize {
            let buffer_row = current_row + start_line;
            Terminal::move_cursor(&terminal::Position { x: 0, y: current_row as u16 })?;
            Terminal::clear_line()?; // Clear the current line
            
            if let Some(line) = self.buffer.lines.get(buffer_row) {
                Terminal::print(line)?; // Print the line from the buffer
            } else {
                Terminal::print("~")?; // Print a tilde for lines beyond the buffer
            }
        }
        Ok(())
    }

    pub fn render_empty_screen(&self) -> Result<(), std::io::Error> {
        let terminal::Size { height, .. } = Terminal::get_terminal_size()?;
        Buffer::default();
        for current_row in 0..height {
            Terminal::move_cursor(&terminal::Position { x: 0, y: current_row })?; // Move cursor to the beginning of the line
            Terminal::clear_line()?; // Clear the current line
            for current_row in 0..height {
                Terminal::clear_line()?;
                Terminal::print("\r\n")?;
                Terminal::print("~")?;

                if current_row + 3 > height {break;}
            }
        }
        Ok(())
    }
    pub fn render(&self) -> Result<(), std::io::Error> {
        if self.buffer.is_empty() {
            Self::render_empty_screen(&self)?;
        }
        else {
            Self::render_buffer(&self)?;
        }
        Terminal::move_cursor(&terminal::Position::default())?;
        Ok(())
    }
    pub fn load(&mut self, filename: &str) -> Option<&str> {
        if let Some(contents) = std::fs::read_to_string(filename).ok() {
            Self::clear_buffer(self);
            for lines in contents.lines() {
                self.buffer.lines.push(lines.to_string());
            }
            Self::resize(self.buffer.lines.len() as u16, 
                self.buffer.lines.iter().map(|line| line.len()).max().unwrap() as u16).unwrap();
            None
        } 
        else {
            Some("File not found")
        } 
        
    }
    pub fn clear_buffer(&mut self) -> () {
        self.buffer.lines.clear();
    }

    pub fn resize(required_height: u16, required_width: u16) -> Result<(), std::io::Error> {
        Terminal::update_size(required_height, required_width)?;
        Ok(())
    }

    
}
