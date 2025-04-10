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

        for (mut screen_row, buffer_row) in (start_line..end_line).enumerate() {
            Terminal::move_cursor(&terminal::Position { x: 0, y: screen_row as u16 })?;
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(buffer_row) {
                // todo: Break line into chunks that fit within the terminal width
                // if line.len() as u16 > width {
                //     for _ in 1..line.len() as u16 % width {
                //         let chunk = &line[..width as usize];
                //         Terminal::print(chunk)?;
                //         Terminal::move_cursor(&terminal::Position { x: 0, y: screen_row as u16 })?;
                //         screen_row += 1;
                //         buffer_row += 1;
                //     }
                // }
                for chunk in line.as_bytes().chunks(width as usize) {
                    let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
                    Terminal::print(chunk_str)?;
                    screen_row += 1;
                    Terminal::move_cursor(&terminal::Position { x: 0, y: screen_row as u16 })?;

                }
            } else {
                Terminal::print("~")?;
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
            Self::resize(self, self.buffer.lines.len() as u16, 
                self.buffer.lines.iter().map(|line| line.len()).max().unwrap() as u16).unwrap();
            None
        } 
        else {
            Some("File not found")
        } 
        
    }
    pub fn clear_buffer(&mut self) {
        self.buffer.lines.clear();
    }

    pub fn resize(&mut self, required_height: u16, required_width: u16) -> Result<(), std::io::Error> {
        Terminal::update_size(required_height, required_width)?;
        self.buffer.reformat_lines(required_width);
        Self::render(&self)?;
        Ok(())
    }

    
}
