use crossterm::event::{read, Event::{self, Key, Resize}, poll, KeyCode::{self}, KeyEvent, KeyEventKind};

pub mod terminal;
mod view;

use view::{View, Location};
use terminal::{Position, Terminal};
use std::{convert, time::{Duration, Instant}};

pub struct Editor {
    should_quit: bool,
    location : Location,
    view: View,
}


impl Editor {
    #[must_use]
    pub fn default() -> Self {
        Editor { should_quit: false, location: Location {x: 0, y: 1}, view: View::default()}
    }

    pub fn handle_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            self.view.load(filename);
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.handle_args();
       // View::render(&self.view).unwrap();

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
            }
            if self.should_quit {
                break;
            }
            if poll(std::time::Duration::from_millis(500))? {
                let event = read()?;
                self.evaluate_event(&event);
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            kind: KeyEventKind::Press, ..
        }) = event
        {
            match event {
                Resize(new_height, new_width) => {
                    View::resize(*new_height, *new_width).unwrap();
                    self.view.render().unwrap();
                }
                Key(KeyEvent {code, ..})  => {
                    self.evaluate_key_event(code);
                }
                _ => (),
            }
        }
    }

    fn evaluate_key_event(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Esc => {
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
                Terminal::print(" ").unwrap();
                Self::move_cursor_by_key(self, KeyCode::Left).unwrap();
            }
            KeyCode::Char(c) => {
                match c {
                    ' ' => {
                        Terminal::print(stringify!( )).unwrap();
                    }
                    _ => (),
                }
                Self::move_cursor_by_key(self, KeyCode::Right).unwrap();
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
            self.view.render()?; // todo: make navigation work without breaking the view
            Terminal::move_cursor(&Position{
                x: self.location.x as u16,
                y: (self.location.y - self.view.scroll_offset.y) as u16,
            })?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }

    //todo: make cursor not move past end of string line
    // it moves up if line x is bigger than x - 1 when moving up
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
            KeyCode::Left => { // fix going left and rendering when location.y = 1;
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
                    location.y = location.y.saturating_sub(1);
                    
                    // Scroll up if cursor goes beyond visible area
                    if location.y < scroll_offset.y {
                        scroll_offset.y = scroll_offset.y.saturating_sub(1);
                        self.view.render()?;
                    }
                }
            }
            KeyCode::Down => {
                if location.y < buffer_height.saturating_sub(1) {
                    location.y = location.y.saturating_add(1);

                    // Scroll down if cursor goes beyond visible area
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
}

