use std::collections::VecDeque;

use crate::{action::Action, utils::popup_area, view::View, prfitem::PrfItem};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
    Frame,
};
use ratatui_input::{Input, InputState};
use tracing::debug;
#[derive(Default)]
enum Mode {
    Input,
    #[default]
    Normal,
}
#[derive(Default)]
pub struct SubScription {
    focus: bool,
    mode: Mode,
    input_popua: bool,
    input_state: InputState,
    input_help: String,
    actions: VecDeque<Action>,
    pref_list: Vec<PrfItem>,
}

impl SubScription {
    pub fn new() -> Self {
        Self {
            input_help: "输入".to_string(),
            ..Default::default()
        }
    }

    pub fn normal_event(&mut self, key: KeyEvent) -> Option<Event> {
        match key.code {
            KeyCode::Char('a') => {
                debug!("a");
                self.input_popua = !self.input_popua;
                None
            }
            KeyCode::Char('i') => {
                self.input_help = "输入中， 按 Esc 退出编辑".to_string();
                self.mode = Mode::Input;
                None
            }
            _ => Some(Event::Key(key)),
        }
    }

    pub fn input_event(&mut self, key: KeyEvent) -> Option<Event> {
        debug!("subscripiton:{:?}", key.code);
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.input_help = "输入".to_string();
                None
            }
            KeyCode::Enter => {
                debug!("enter");
                self.actions
                    .push_back(Action::SubScription(self.input_state.text().to_string()));
                None
            }
            _ => {
                self.input_state.handle_message(key.into());
                None
            }
        }
    }
}

impl View for SubScription {
    fn draw_menu(&mut self, f: &mut Frame, area: Rect) {
        let mut b = Block::bordered().title("订阅");
        if self.focus {
            b = b.border_style(Style::default().fg(Color::Yellow));
        }
        let p = Paragraph::new("订阅").block(b);
        f.render_widget(p, area);
        // todo!();
    }

    fn draw_detail(&mut self, f: &mut Frame, area: Rect) {
        let p = Paragraph::new("订阅-详情页");
        f.render_widget(p, area);
        if self.input_popua {
            let b = Block::bordered().title(self.input_help.clone());
            let area = popup_area(f.area(), 60, 10);
            f.render_widget(Clear, area);
            let input = Input::default();
            f.render_widget(b.clone(), area);
            let inner_area = b.inner(area);
            f.render_stateful_widget(input, inner_area, &mut self.input_state);
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Event> {
        if let Event::Key(key) = event.clone() {
            if key.kind != event::KeyEventKind::Press {
                return Some(event);
            };
            let handle_event = match self.mode {
                Mode::Normal => self.normal_event(key),
                Mode::Input => self.input_event(key),
            };
            return handle_event;
        }
        Some(event)
    }

    fn is_focus(&self) -> bool {
        self.focus
    }

    fn set_focus(&mut self) {
        self.focus = !self.focus
    }

    fn name(&self) -> String {
        "订阅".to_string()
    }

    fn length(&self) -> u16 {
        20
    }

    fn get_events(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }

    fn update(&mut self, action: Option<Action>) -> Result<()> {
        if action.is_none() {
            return Ok(());
        }

        match action.unwrap() {
            Action::UpdatePrfList(items) => {
                self.pref_list = items;
                Ok(())
            },
            _ => Ok(()),
        }
    }
}
