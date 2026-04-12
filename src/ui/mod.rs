//! The UI module contains reusable components and layouts for Ratatui.
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

pub trait App {
    fn render(&mut self, f: &mut Frame, area: Rect); //draw the app's UI 
    fn handle_input(&mut self, key: KeyEvent); //handel kep presses  
    fn name(&self) -> &str;
}
