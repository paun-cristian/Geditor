#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
pub mod editor;
use editor::Editor;

// use egui for the GUI
// do error handling
fn main() {
    Editor::default().run();
}