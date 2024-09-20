use crossterm::event::{read, Event::{self, Key}, KeyCode::{self}, KeyEvent, KeyEventKind};

pub mod terminal;
mod view;

use view::View;
use terminal::{Position, Terminal};
#[derive(Default, Clone, Copy)]
pub struct Location {
    x: usize,
    y: usize,
}
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
        Terminal::move_cursor(&Position{x: 0, y: 0})?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        }
        else {
            // self.view.render()?; // todo: make navigation work without breaking the view
            Terminal::move_cursor(&terminal::Position{
                x: self.location.x as u16,
                y: self.location.y as u16,
            })?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }

    pub fn move_cursor_by_key(&mut self, keycode: KeyCode) -> Result<(), std::io::Error> {
        let mut l = self.location;
        let s = Terminal::get_terminal_size()?;
        match keycode {
            KeyCode::Right => {
                l.x = core::cmp::min(s.width.saturating_sub(1) as usize, l.x.saturating_add(1));
            }
            KeyCode::Left => {
                l.x = l.x.saturating_sub(1);
            }
            KeyCode::Up => {
                l.y = l.y.saturating_sub(1);
            }
            KeyCode::Down => {
                l.y = core::cmp::min(s.height.saturating_sub(1) as usize, l.y.saturating_add(1));
            }
            KeyCode::Home => {
                l.x = 0;
            }
            KeyCode::End => {
                l.x = s.width.saturating_sub(1) as usize;
            }
            KeyCode::PageUp => {
                l.y = 0;
            }
            KeyCode::PageDown => {
                l.y = s.height.saturating_sub(1) as usize;
            }
            _ => ()
        }
        self.location = l;
        Ok(())
    }

}

