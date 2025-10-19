use crossterm::event::{poll, read, Event::{self, Key, Resize}, KeyCode::{self}, KeyEvent, KeyEventKind, KeyModifiers};
pub mod terminal;
mod view;
mod file;

use file::File;
use view::{View, Location};
use terminal::{Position, Terminal};
use std::time::{Duration, Instant};
pub struct Editor {
    should_quit: bool,
    location : Location,
    view: View,
    file: File,
    terminal: std::io::Stdout,
}


impl Editor {
    #[must_use]
    pub fn default() -> Self {
        Editor { should_quit: false, location: Location {x: 0, y: 1}, view: View::default(), file: File::default(), terminal: std::io::stdout()} 
    }

    pub fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.file.filename = filename.clone(); 
            self.view.load(filename);
        }
    }
    pub fn save(&mut self){
        if self.file.filename.is_empty() {
            println!("No filename specified");
            return;
        }
        let contents : String = self.view.buffer.lines.iter().map(|line| line.to_string()).collect::<Vec<String>>().join("\n");
        std::fs::write(&self.file.filename, contents).expect("Unable to write file");
        self.file.saved = true;
        self.file.modified = true;
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
        self.terminal = std::io::stdout();
        //Terminal::set_background_color(self.terminal, terminal::Color::Black).unwrap();
        
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        Terminal::enable_raw_mode()?;
        let mut last_render = Instant::now();
        View::render(&self.view)?;
        
        loop {
            if last_render.elapsed() >= Duration::from_millis(16) { //render each sec 60times
                self.refresh_screen()?;
                last_render = Instant::now();
                self.save();
            }
            if self.should_quit {
                break;
            }
            if poll(std::time::Duration::from_millis(500))? {
                let event = read()?;
                self.evaluate_event(&event).unwrap();
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) -> Result<(), String> {
        if let Key(KeyEvent {
            kind: KeyEventKind::Press, ..
        }) = event
        {
            match event {
                Resize(new_height, new_width) => {
                    return Ok(View::resize(&mut self.view, *new_height, *new_width).map_err(|e| e.to_string())?);
                }
                Key(KeyEvent { code, modifiers, .. }) => {
                    if modifiers.is_empty() {
                        self.evaluate_key_event(code);
                    } else {
                        self.evaluate_modifiers(modifiers, code);
                    }
                }
                _ => return Err(format!("Invalid event: {:?}", event)),
            }
        }
        Ok(())
    }

    
    fn evaluate_key_event(&mut self, code: &KeyCode) {
        self.file.modified = true;
        match code {
            KeyCode::Esc => {
                self.save();
                self.should_quit = true;
            }
            KeyCode::Right 
            | KeyCode::Left
            | KeyCode::Up
            | KeyCode::Down
            | KeyCode::Home
            | KeyCode::End
            | KeyCode::PageUp
            | KeyCode::PageDown => {
                Self::move_cursor_by_key(self, *code).unwrap();
            }
            KeyCode::Backspace => {
                Self::backspace(self);
            }
            KeyCode::Enter => {
                Self::enter(self);
            }
            KeyCode::Char(c) => {
                match c {
                    ' ' => {
                        Self::print_to_buffer(self, c);
                    }
                    _ => {
                        Self::print_to_buffer(self, c);
                    }
                }
                Self::move_cursor_by_key(self, KeyCode::Right).unwrap();
            }
            _ => (),
        }
    }

    fn evaluate_modifiers(&mut self, modifiers: &KeyModifiers, code: &KeyCode) {
        match modifiers {
            &KeyModifiers::CONTROL => {
                match code {
                    KeyCode::Char('s') => {
                        self.save();
                    }
                    KeyCode::Char('a') => {
                        self.location.x = 0;
                    }
                    KeyCode::Right => {
                        let line = &self.view.buffer.lines[self.location.y];
                        if let Some(space_index) = line[self.location.x..].find(' ') {
                            self.location.x += space_index + 1;
                        } else {
                            self.location.x = line.len();
                        }
                        if self.location.x == line.len() {
                            self.location.y = self.location.y.saturating_add(1);
                            self.location.x = 0;
                        }
                    }
                    KeyCode::Left => {
                        let line = &self.view.buffer.lines[self.location.y][..self.location.x];
                        let temp_loc = line.rfind(' ');

                        match temp_loc {
                            Some(index) => self.location.x = index,
                            None => {
                                if self.location.y > 0 && self.location.x == 0 {
                                    self.location.y = self.location.y.saturating_sub(1);
                                    self.location.x = self.view.buffer.lines[self.location.y].len();
                                }
                                else {
                                    self.location.x = 0;
                                }
                            }
                        }
                    }
                        _ => (),
                }
            }
            _ => (),
        }
    }
    
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor(&Position::default())?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor(&Position::default())?;
            Terminal::print("Goodbye.\r\n")?;
        }
        else {
            self.view.render()?; 
            Terminal::move_cursor(&Position{
                x: self.location.x as u16,
                y: (self.location.y - self.view.scroll_offset.y) as u16,
            })?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }

    
    pub fn move_cursor_by_key(&mut self, keycode: KeyCode) -> Result<(), std::io::Error> {
        let terminal_size = Terminal::get_terminal_size()?;
        let buffer_height = self.view.buffer.lines.len();
        let location = &mut self.location;
        let scroll_offset = &mut self.view.scroll_offset;

        match keycode {
            KeyCode::Right => {
                let current_line_length = self.view.buffer.lines[location.y].len();

                // If we're at the end of the line, move to the next line
                if location.x == current_line_length {
                    // Move to the next line and reset x to 0 if there's another line
                    if location.y < buffer_height.saturating_sub(1) {
                        location.y = location.y.saturating_add(1);
                        location.x = 0;
                    }
                    
                    if location.y >= scroll_offset.y + terminal_size.height as usize {
                        scroll_offset.y = scroll_offset.y.saturating_add(1);
                        self.view.render()?;
                    }
                } else {
                    // Otherwise, just move to the right within the same line
                    location.x = location.x.saturating_add(1);
                }

            }
            KeyCode::Left => {
                if location.x > 0 {
                    location.x = location.x.saturating_sub(1);
                } else 
                    if location.y > 0 {
                        location.y = location.y.saturating_sub(1);

                        if location.y < scroll_offset.y {
                            scroll_offset.y = scroll_offset.y.saturating_sub(1);
                            self.view.render()?;
                        }
                        else {
                            let prev_line = self.view.buffer.lines[location.y].len();

                            location.x = prev_line;
                            }
                    }
            }
            KeyCode::Up => {
                if location.y > 0 {
                    
                    //let current_line_length = self.view.buffer.lines[location.y].len();
                    let prev_line_len = self.view.buffer.lines[location.y - 1].len();
                    
                    location.y = location.y.saturating_sub(1);

                    if location.x > prev_line_len {
                        location.x = prev_line_len;
                    }
                    if location.y < scroll_offset.y {
                        scroll_offset.y = scroll_offset.y.saturating_sub(1);
                        self.view.render()?;
                    }
                    
                }
            }
            KeyCode::Down => { 
                if location.y < buffer_height.saturating_sub(1) {

                    //let current_line_length = self.view.buffer.lines[location.y].len();
                    let next_line_len = self.view.buffer.lines[location.y + 1].len();
                    
                    location.y = location.y.saturating_add(1);

                    if location.x > next_line_len {
                        location.x = next_line_len;
                        
                    }
                    if location.y >= scroll_offset.y + terminal_size.height as usize {
                        scroll_offset.y = scroll_offset.y.saturating_add(1);
                        self.view.render()?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn enter(&mut self){
        let current_line = &mut self.view.buffer.lines[self.location.y];
        if self.location.x == current_line.len() {
            self.view.buffer.lines.insert(self.location.y + 1, String::new());
            Self::move_cursor_by_key(self, KeyCode::Down).unwrap();
        } else {
            let new_line = current_line.split_off(self.location.x);
            self.view.buffer.lines.insert(self.location.y + 1, new_line);
            Self::move_cursor_by_key(self, KeyCode::Down).unwrap();
        }
        self.location.x = 0;    
    }

    pub fn backspace(&mut self){
        let line = &mut self.view.buffer.lines[self.location.y];
        let mut moved : bool = false;
        let mut prev_len = 0;

        if self.location.x > 0 {
            line.remove(self.location.x - 1);
            self.move_cursor_by_key(KeyCode::Left).unwrap();
        } else if self.location.y > 0 {
            moved = true;
            let (prev_line, current_line) = self.view.buffer.lines.split_at_mut(self.location.y);
            prev_len = prev_line[self.location.y - 1].len();
            prev_line[self.location.y - 1].push_str(&current_line[0]);
            self.view.buffer.lines.remove(self.location.y);
            self.view.render().unwrap();
        } 
        if moved {
            self.move_cursor_by_key(KeyCode::Up).unwrap();  
            self.location.x = prev_len;
        }
    }

    pub fn print_to_buffer(&mut self, c: &char) {
        let _terminal = Terminal::get_terminal_size().unwrap();
    //    let line_len = self.view.buffer.lines[self.location.y].len();
    
        if self.location.x as u16 >= _terminal.width {
            Self::move_cursor_by_key(self, KeyCode::Down).unwrap();
            self.location.x = 0;            
        }
    
        let line = &mut self.view.buffer.lines[self.location.y];
        line.insert(self.location.x, *c);
    
        if line.len() as u16 > _terminal.width {
            let next_line_content = line.split_off(_terminal.width as usize);
    
            if self.location.y + 1 < self.view.buffer.lines.len() {
                self.view.buffer.lines[self.location.y + 1].insert_str(0, &next_line_content);
            } else {
                self.view.buffer.lines.push(next_line_content);
            }
        }
        self.view.render().unwrap();
    }

}


