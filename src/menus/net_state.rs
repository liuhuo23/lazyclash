use crate::{action::Action, components::Component};
use color_eyre::{owo_colors::OwoColorize, Result};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, style::Stylize, widgets::Block, Frame};
use tokio::sync::mpsc::UnboundedSender;

use super::Menu;

pub struct NetState {
    is_active: bool,
    pub action_tx: Option<UnboundedSender<Action>>,
    detail_view: Option<Box<dyn Component>>,
    pub last_events: Vec<KeyEvent>,
}

impl NetState {
    pub fn new(is_active: bool) -> Self {
        Self {
            is_active: is_active,
            action_tx: None,
            detail_view: None,
            last_events: vec![],
        }
    }
}

impl Component for NetState {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::bordered().title("[网络流量]");
        if self.is_active {
            block = block.white();
        } else {
            block = block.gray();
        }
        frame.render_widget(block, area);
        Ok(())
    }
}

impl Menu for NetState {
    fn get_length(&self) -> u16 {
        20
    }

    fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_detail(&mut self) -> &mut Option<Box<dyn Component>> {
        &mut self.detail_view
    }
}
