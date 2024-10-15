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
    pub horizontal_scroll_state: ScrollbarState,
    pub horizontal_scroll: usize,
    pub is_active: bool,
    pub char_index: u16,
    pub postion: u16,
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
        let area = popup_area(frame.area(), 60, 10);
        let uri = Line::raw(self.value.clone());
        let paragraph = Paragraph::new(uri)
            .gray()
            .block(block)
            .left_aligned()
            // .wrap(Wrap { trim: true })
            .scroll((0, self.horizontal_scroll as u16));
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
        match self.mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            Mode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            Mode::Editting => {
                if self.postion >= area.width - 3 {
                    self.postion = area.width - 3;
                }
                frame.set_cursor_position(Position::new(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    area.x + self.postion as u16 + 1,
                    // Move one line down, from the border to the input line
                    area.y + 1,
                ));
            }
        };
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some("⬅️"))
                .end_symbol(Some("➡️")),
            area.inner(Margin {
                vertical: 0,
                horizontal: 1,
            }),
            &mut self.horizontal_scroll_state,
        );
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
                KeyCode::Left => {
                    self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
                    self.horizontal_scroll_state = self
                        .horizontal_scroll_state
                        .position(self.horizontal_scroll);
                    Action::Update
                }
                KeyCode::Right => {
                    if self.postion < self.value.len() as u16 {
                        self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
                        self.horizontal_scroll_state = self
                            .horizontal_scroll_state
                            .position(self.horizontal_scroll);
                    }
                    self.postion = self.postion.saturating_add(1);
                    if self.char_index < self.value.len() as u16 {
                        self.char_index = self.char_index.saturating_add(1);
                    }
                    Action::Update
                }
                KeyCode::Backspace => {
                    self.value.pop();
                    self.postion = self.postion.saturating_sub(1);
                    self.char_index = self.char_index.saturating_sub(1);
                    Action::Update
                }
                KeyCode::Char(v) => {
                    self.value.push(v);
                    self.postion = self.postion.saturating_add(1);
                    self.char_index = self.char_index.saturating_add(1);
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
