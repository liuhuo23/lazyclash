use crate::action::Action;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{error, info};

use super::Component;

#[derive(Debug, Default, Clone)]
enum Mode {
    #[default]
    Normal,
    Editting,
}

#[derive(Debug, Default, Clone)]
pub struct LInput<'a> {
    value: String,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keymap: HashMap<KeyEvent, Action>,
    pub text: Vec<String>,
    pub last_events: Vec<KeyEvent>,
    pub is_multiline: bool,
    pub mode: Mode,
    pub block: Block<'a>,
}

impl<'a> LInput<'a> {
    fn new() -> Self {
        Self {
            is_multiline: false,
            ..Default::default()
        }
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn submit(&mut self, func: fn(v: &mut String) -> Result<()>) -> Result<()> {
        func(&mut self.value)
    }
}

impl<'a> Component for LInput<'a> {
    fn draw(&mut self, frame: &mut Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let input = Paragraph::new(self.value.as_str())
            .style(match self.mode {
                Mode::Normal => Style::default(),
                Mode::Editting => Style::default().fg(Color::Yellow),
            })
            .block(self.block.clone());
        frame.render_widget(input, area);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.last_events.push(key);
        let action = match self.mode {
            Mode::Normal => return Ok(None),
            Mode::Editting => match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    Action::EnterNormal
                }
                KeyCode::Enter => {
                    if !self.is_multiline {
                        self.mode = Mode::Normal;
                        self.submit(|v| {
                            info!("{}", v);
                            Ok(())
                        })?;
                        Action::EnterNormal
                    } else {
                        self.value.push('\n');
                        Action::Update
                    }
                }
                KeyCode::Left => Action::Update,
                KeyCode::Right => Action::Update,
                KeyCode::Backspace => {
                    self.value.pop();
                    Action::Update
                }
                KeyCode::Char(v) => {
                    self.value.push(v);
                    Action::Update
                }
                _ => Action::Update,
            },
        };
        Ok(Some(action))
    }
}
