use crossterm::event::{Event, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::menu::Menu;
#[derive(Default)]
pub struct Version {
    focus: bool,
}

impl Version {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Menu for Version {
    fn draw_menu(&mut self, f: &mut Frame, area: Rect) {
        let mut b = Block::bordered().title("Version");
        if self.focus {
            b = b.border_style(Style::default().fg(Color::Yellow));
        }
        let p = Paragraph::new("Version 0.0.1").block(b);
        f.render_widget(p, area);
    }

    fn draw_detail(&mut self, f: &mut Frame, area: Rect) {
        let p = Paragraph::new("版本-详情页");
        f.render_widget(p, area)
    }

    fn handle_event(&mut self, event: Event) {}

    fn is_focus(&self) -> bool {
        self.focus
    }

    fn set_focus(&mut self) {
        self.focus = !self.focus;
    }

    fn name(&self) -> String {
        "版本".to_string()
    }
}
