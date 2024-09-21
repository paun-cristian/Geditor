mod buffer;
use buffer::Buffer;
use super::{terminal, Terminal};

#[derive(Default, Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Clone, Copy)]
struct ScrollOffset {
    pub x: usize,
    pub y: usize,
}
#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
    pub scroll_offset: ScrollOffset,
}

impl View {
    fn default() -> Self {
        Self { buffer: Buffer::default(), scroll_offset: ScrollOffset { x: 0, y: 0 } }
    }
    pub fn render_buffer(&self) -> Result<(), std::io::Error> {
        let terminal::Size { mut height, width} = Terminal::get_terminal_size()?;
        if self.buffer.lines.len() as u16 > height {
            height = self.buffer.lines.len() as u16;
            Terminal::update_size(self.buffer.lines.len() as u16, width)?;
        }
        for current_row in 0..height as u16 {
            Terminal::move_cursor(&terminal::Position { x: 0, y: current_row })?; // Move cursor to the beginning of the line
            Terminal::clear_line()?; // Clear the current line
            if let Some(line) = self.buffer.lines.get(current_row as usize) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                }
             else {
                Terminal::print("~")?;
            }
            if current_row + 3 > height {break;}
        }
        println!("Rows: {}", height); // shows 298, there are actually 298
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
