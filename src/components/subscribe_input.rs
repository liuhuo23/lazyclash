use crate::action::Action;
use crate::utils::popup_area;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use ratatui_input::{Input, InputState};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, info};

use super::Component;

#[derive(Debug, Default, Clone)]
enum Mode {
    #[default]
    Normal,
    Editting,
}

#[derive(Debug, Default)]
pub struct SubInput {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keymap: HashMap<KeyEvent, Action>,
    pub last_events: Vec<KeyEvent>,
    pub is_multiline: bool,
    pub mode: Mode,
    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll: usize,
    pub is_active: bool,
    pub input_state: InputState,
}

impl SubInput {
    fn new() -> Self {
        Self {
            is_multiline: false,
            is_active: false,
            ..Default::default()
        }
    }

    fn get_value(&self) -> String {
        self.input_state.text().to_string()
    }

    fn submit(&mut self, func: fn(v: &mut String) -> Result<()>) -> Result<()> {
        func(&mut self.get_value())
    }
}

impl Component for SubInput {
    fn draw(&mut self, frame: &mut Frame, _area: ratatui::prelude::Rect) -> Result<()> {
        let block = Block::bordered().title("订阅");
        let area = popup_area(frame.area(), 60, 10);
        frame.render_widget(Clear, area);
        let input = Input::default();
        let inner_area = block.inner(area);
        frame.render_widget(block, area);
        frame.render_stateful_widget(input, inner_area, &mut self.input_state);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.last_events.push(key);
        let action = match self.mode {
            Mode::Normal => return Ok(None),
            Mode::Editting => match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    Action::ExitInput
                }
                _ => {
                    self.input_state.handle_message(key.into());
                    Action::Update
                }
            },
        };
        Ok(Some(action))
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match self.mode {
            Mode::Normal => {
                match action {
                    Action::EnterSubscribe if !self.is_active => {
                        self.is_active = true;
                        self.mode = Mode::Editting;
                    }
                    Action::EnterSubscribe if self.is_active => {
                        self.is_active = false;
                    }
                    Action::ExitSubscribe(ulr) => {
                        debug!("{ulr}");
                        tokio::spawn(async { todo!() });
                    }
                    _ => {}
                };
            }
            _ => {}
        };
        Ok(None)
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }
}
