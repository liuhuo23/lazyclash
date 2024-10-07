use crate::details::version_detail::VersionDetail;
use crate::{action::Action, components::Component};
use color_eyre::eyre::Ok;
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use tokio::sync::mpsc::UnboundedSender;

use super::Menu;

pub struct Subscribe {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub detail_view: Option<Box<dyn Component>>,
    is_active: bool,
    last_events: Vec<KeyEvent>,
    show_input: bool,
    subscribe_url: Option<String>,
}

impl Subscribe {
    pub fn new(is_active: bool) -> Self {
        Self {
            detail_view: Some(Box::new(VersionDetail::new())),
            is_active: is_active,
            action_tx: None,
            last_events: vec![],
            show_input: false,
            subscribe_url: None,
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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}