mod buffer;
use buffer::Buffer;
use super::{terminal, Terminal};

#[derive(Default)]
pub struct View {
    pub buffer: Buffer
}
// todo: fix render_buffer
impl View {
    pub fn render_buffer(&self) -> Result<(), std::io::Error> {
        let terminal::Size { mut height, ..} = Terminal::get_terminal_size()?;
        if self.buffer.lines.len() as u16 > height {
            height = self.buffer.lines.len() as u16;
        }
        for current_row in 0..height as u16 {
            Terminal::move_cursor(&terminal::Position { x: 0, y: current_row })?; // Move cursor to the beginning of the line
            Terminal::clear_line()?; // Clear the current line
            if let Some(line) = self.buffer.lines.get(current_row as usize) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                /* if line.len() as u16 > width {
                    let mut x = 0;
                    let mut current_column: u16 = width;
                    while line.len() as u16 > width {
                        Terminal::print(&line[x..width as usize])?;
                        Terminal::move_cursor(&Position { x: 0, y: current_row + 1 })?;
                        Terminal::clear_line()?;
                        x += width as usize;
                        current_column = (line.len() - x) as u16;
                    }
                    Terminal::print(&line[x..current_column as usize])?; */
                }
             else {
                Terminal::print("~")?;
            }
            if current_row + 3 > height {break;}
        }
        // println!("Rows: {}", self.buffer.lines.len()); // shows 298, there are actually 298
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
            Self::render_empty_screen(&self)
        }
        else {
            Self::render_buffer(&self)
        }
    }
    pub fn load(&mut self, filename: &str) -> Option<&str> {
        if let Some(contents) = std::fs::read_to_string(filename).ok() {
            Self::clear_buffer(self);
            for lines in contents.lines() {
                self.buffer.lines.push(lines.to_string());
            }
            None
        } 
        else {
            Some("File not found")
        } 
        
    }
    pub fn clear_buffer(&mut self) -> () {
        self.buffer.lines.clear();
    }
}
