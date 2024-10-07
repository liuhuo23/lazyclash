use crate::details::version_detail::VersionDetail;
use crate::{action::Action, components::Component};
use color_eyre::eyre::Ok;
use color_eyre::eyre::Result;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use tokio::sync::mpsc::UnboundedSender;

use super::Menu;

pub struct Subscribe {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub detail_view: Option<Box<dyn Component>>,
    is_active: bool,
}

impl Subscribe {
    pub fn new(is_active: bool) -> Self {
        Self {
            detail_view: Some(Box::new(VersionDetail::new())),
            is_active: is_active,
            action_tx: None,
        }
    }
}

impl Component for Subscribe {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let mut block = Block::bordered().title("[订阅]");
        if self.is_active {
            block = block.white();
        } else {
            block = block.gray();
        }
        frame.render_widget(block, area);
        Ok(())
    }
}

impl Menu for Subscribe {
    fn get_length(&self) -> u16 {
        3
    }

    fn get_detail(&mut self) -> &mut Option<Box<dyn Component>> {
        &mut self.detail_view
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
    fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}
