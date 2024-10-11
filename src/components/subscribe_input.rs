use crate::action::Action;
use crate::utils::popup_area;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
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

#[derive(Debug, Default, Clone)]
pub struct SubInput {
    value: String,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keymap: HashMap<KeyEvent, Action>,
    pub text: String,
    pub last_events: Vec<KeyEvent>,
    pub is_multiline: bool,
    pub mode: Mode,
    pub is_active: bool,
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
        self.value.clone()
    }

    fn submit(&mut self, func: fn(v: &mut String) -> Result<()>) -> Result<()> {
        func(&mut self.value)
    }
}

impl Component for SubInput {
    fn draw(&mut self, frame: &mut Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let block = Block::bordered().title("订阅");
        let area = popup_area(frame.area(), 60, 20);
        let uri = Line::raw(self.value.clone());
        let paragraph = Paragraph::new(uri)
            .gray()
            .block(block)
            .left_aligned()
            .wrap(Wrap { trim: true });
        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(paragraph, area);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.last_events.push(key);
        let action = match self.mode {
            Mode::Normal => return Ok(None),
            Mode::Editting => match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    Action::Update
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
