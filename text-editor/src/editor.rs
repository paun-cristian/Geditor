use crossterm::event::{read, Event::{self, Key}, KeyCode::{self}, KeyEvent, KeyEventKind};

pub mod terminal;
mod view;

use view::{View, Location};
use terminal::{Position, Terminal};

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
        View::render(&self.view)?;
        
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            kind: KeyEventKind::Press, ..
        }) = event
        {
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
    pub fn move_cursor_by_key(&mut self, keycode: KeyCode) -> Result<(), std::io::Error> {
        let terminal_size = Terminal::get_terminal_size()?;
        let buffer_height = self.view.buffer.lines.len();
        let location = &mut self.location;
        let scroll_offset = &mut self.view.scroll_offset;

        match keycode {
            KeyCode::Right => {
                location.x = core::cmp::min(self.view.buffer.lines[location.y].len(), location.x + 1);
            }
            KeyCode::Left => {
                location.x = location.x.saturating_sub(1);
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

