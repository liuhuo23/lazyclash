use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, components::Component};

#[derive(Debug, Default, Clone)]
pub struct VersionDetail {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keymap: HashMap<KeyEvent, Action>,
}

impl VersionDetail {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for VersionDetail {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        Ok(())
    }
}
