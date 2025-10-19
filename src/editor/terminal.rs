use crossterm::{
    style::*, execute, cursor::{Hide, Show}, event::read, queue, style::Print, terminal::{self, disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetTitle}
};
use std::io::{stdout, Stdout, Write};

pub struct Terminal;

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16
}
impl Default for Size {
    fn default() -> Self {
        Self {width: 1280, height: 720}
    }
}

#[derive(Clone, Copy, Default)]
pub struct Position {    
    pub x: u16,
    pub y: u16
}

impl Position {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }

}

impl Terminal {
    pub fn enable_raw_mode() -> Result<(), std::io::Error> {
        enable_raw_mode().unwrap();
        Ok(())
    }
    pub fn disable_raw_mode() -> Result<(), std::io::Error> {
        disable_raw_mode().unwrap();
        Ok(())
    }
    pub fn set_background_color(mut terminal: Stdout, color: Color) -> Result<(), std::io::Error> {
        execute!(terminal, SetBackgroundColor(color))?;
        Ok(())
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        Terminal::enable_raw_mode()?;
        Terminal::clear_screen()?;
        queue!(stdout(), SetTitle("Geditor"))?;
        Terminal::move_cursor(&Position::default())?;
        Terminal::execute()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        Terminal::disable_raw_mode()
    }   
    pub fn read_key() -> Result<(), std::io::Error> {
        read().unwrap();
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Terminal::execute()
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Terminal::execute()
    }
    pub fn move_cursor(position : &Position) -> Result<(), std::io::Error> {
        queue!(stdout(), crossterm::cursor::MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn get_terminal_size() -> Result<Size, std::io::Error> {
        let (width, height) = size().unwrap();
        Ok(Size { width, height })
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn print_string(string: String) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn message() -> Result<(), std::io::Error> {
        let s = Terminal::get_terminal_size()?;
        Terminal::show_cursor()?;
        Terminal::move_cursor(&Position { x: 0, y: s.height })?;
        Terminal::print("Welcome to editor ver.1\r")?;
        Ok(())
    }
    pub fn show_cariet() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn hide_cariet() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn update_size(height: u16, width: u16) -> Result<(), std::io::Error> {
        let s = &mut self::Size { height, width };
        s.height = height;
        s.width = width;
        Ok(())
    }
    pub fn update_position(x: u16, y: u16) -> Result<(), std::io::Error> {
        self::Position { x, y };
        Ok(())
    }
    
}
