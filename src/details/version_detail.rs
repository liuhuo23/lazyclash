use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, components::Component};

#[derive(Debug, Default, Clone)]
pub struct VersionDetail {
    action_tx: Option<UnboundedSender<Action>>,
    keymap: HashMap<KeyEvent, Action>,
    name: String,
    scroll: u16,
}

impl VersionDetail {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Component for VersionDetail {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let block = Block::bordered().gray().title("[版本详情]".bold());
        let lines = vec![Line::raw("version: 0.0.1")];
        let paragraph = Paragraph::new(lines).block(block).scroll((self.scroll, 0));
        frame.render_widget(paragraph, area);
        Ok(())
    }
}
