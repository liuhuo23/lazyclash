use crate::menu::Menu;
use crossterm::event::Event;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
#[derive(Default)]
pub struct SubScription {
    focus: bool,
}
impl SubScription {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Menu for SubScription {
    fn draw_menu(&mut self, f: &mut Frame, area: Rect) {
        let mut b = Block::bordered().title("订阅");
        if self.focus {
            b = b.border_style(Style::default().fg(Color::Yellow));
        }
        let p = Paragraph::new("订阅").block(b);
        f.render_widget(p, area);
    }

    fn draw_detail(&mut self, f: &mut Frame, area: Rect) {
        let p = Paragraph::new("订阅-详情页");
        f.render_widget(p, area)
    }

    fn handle_event(&mut self, event: Event) {}

    fn is_focus(&self) -> bool {
        self.focus
    }

    fn set_focus(&mut self) {
        self.focus = !self.focus
    }

    fn name(&self) -> String {
        "订阅".to_string()
    }
}
