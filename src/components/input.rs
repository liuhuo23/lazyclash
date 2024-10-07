use color_eyre::Result;
use crossterm::event::KeyEvent;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

use super::Component;

#[derive(Debug, Default, Clone)]
pub struct LInput {
    value: String,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keymap: HashMap<KeyEvent, Action>,
    pub text: Vec<String>,
    pub last_events: Vec<KeyEvent>,
}

impl LInput {
    fn new() -> Self {
        Self::default()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl Component for LInput {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        todo!()
    }
}
