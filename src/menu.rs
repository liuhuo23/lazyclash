use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};
pub mod subscription;
pub mod version;

pub trait Menu {
    fn draw_menu(&mut self, f: &mut Frame, area: Rect);

    fn draw_detail(&mut self, f: &mut Frame, area: Rect);

    fn handle_event(&mut self, event: Event);

    fn is_focus(&self) -> bool {
        false
    }

    fn set_focus(&mut self);

    fn name(&self) -> String;
}
