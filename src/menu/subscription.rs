use std::collections::VecDeque;

use crate::{action::Action, prfitem::PrfItem, utils::popup_area, view::View};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Styled, Stylize,
    },
    text::Line,
    widgets::{Block, Clear, HighlightSpacing, List, ListItem, ListState, Paragraph},
    Frame,
};
use ratatui_input::{Input, InputState};
use tracing::debug;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(Default)]
enum Mode {
    Input,
    #[default]
    Normal,
    Detail,
}

#[derive(Default)]
pub struct PrfItemList {
    items: Vec<PrfItem>,
    state: ListState,
}

impl PrfItemList {
    pub fn set_items(&mut self, items: Vec<PrfItem>) {
        self.items = items;
    }
}

#[derive(Default)]
pub struct SubScription {
    focus: bool,
    mode: Mode,
    input_popua: bool,
    input_state: InputState,
    input_help: String,
    actions: VecDeque<Action>,
    pref: PrfItemList,
    selected_index: usize,
}

impl SubScription {
    pub fn new() -> Self {
        let mut actions = VecDeque::new();
        actions.push_back(Action::SubScriptionUpdate);
        Self {
            input_help: "输入".to_string(),
            actions,
            ..Default::default()
        }
    }

    fn select_next(&mut self) {
        self.pref.state.select_next();
    }
    fn select_previous(&mut self) {
        self.pref.state.select_previous();
    }

    fn selected(&mut self) {
        if let Some(i) = self.pref.state.selected() {
            self.selected_index = i;
            // self.actions
            //     .push_back(Action::SelectedItem(item.uid.clone().unwrap()));
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
            KeyCode::Down => {
                self.select_next();
                None
            }
            KeyCode::Up => {
                self.select_previous();
                None
            }
            KeyCode::Char(' ') => {
                self.selected();
                debug!("触发选中");
                None
            }
            KeyCode::Enter => {
                self.mode = Mode::Detail;
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

    pub fn detail_event(&mut self, key: KeyEvent) -> Option<Event> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                None
            }
            _ => None,
        }
    }
}

impl View for SubScription {
    fn draw_menu(&mut self, f: &mut Frame, area: Rect) {
        let mut b = Block::bordered().title("订阅");
        if self.focus {
            b = b.border_style(Style::default().fg(Color::Yellow));
        }
        let items: Vec<ListItem> = self
            .pref
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let line = if self.selected_index == i {
                    Line::styled(
                        format!(" ✓ {}", item.name.as_ref().map_or("config", |f| &f)),
                        COMPLETED_TEXT_FG_COLOR,
                    )
                } else {
                    Line::styled(
                        format!(" ☐ {}", item.name.as_ref().map_or("config", |f| &f)),
                        TEXT_FG_COLOR,
                    )
                };
                ListItem::new(line)
            })
            .collect();
        let list = List::new(items)
            .block(b)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        f.render_stateful_widget(list, area, &mut self.pref.state);
        // todo!();
    }

    fn draw_detail(&mut self, f: &mut Frame, area: Rect) {
        let b = match self.mode {
            Mode::Normal | Mode::Input => Block::bordered(),
            Mode::Detail => Block::bordered().border_style(Color::Yellow),
        };
        let inner_area = b.inner(area);
        f.render_widget(b, area);
        let area = inner_area;
        if self.input_popua {
            let b = Block::bordered().title(self.input_help.clone());
            let area = popup_area(f.area(), 60, 10);
            f.render_widget(Clear, area);
            let input = Input::default();
            f.render_widget(b.clone(), area);
            let inner_area = b.inner(area);
            f.render_stateful_widget(input, inner_area, &mut self.input_state);
        }
        if self.pref.state.selected().is_none() {
            return;
        }
        let item = self.pref.items.get(self.pref.state.selected().unwrap());
        if item.is_some() {
            let item = item.unwrap();
            let p = Paragraph::new(format!(
                "{}",
                item.file_data.clone().unwrap_or("没有信息".to_string())
            ));
            f.render_widget(p, area);
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
                Mode::Detail => {
                    self.detail_event(key);
                    self.normal_event(key)
                },
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
                self.pref.set_items(items);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl From<&PrfItem> for ListItem<'_> {
    fn from(value: &PrfItem) -> Self {
        let line = match value.selected.map_or(false, |f| f) {
            false => Line::styled(
                format!(" ☐ {}", value.name.as_ref().map_or("config", |f| &f)),
                TEXT_FG_COLOR,
            ),
            true => Line::styled(
                format!(" ✓ {}", value.name.as_ref().map_or("config", |f| &f)),
                COMPLETED_TEXT_FG_COLOR,
            ),
        };
        ListItem::new(line)
    }
}

// const fn alternate_colors(i: usize) -> Color {
//     if i % 2 == 0 {
//         NORMAL_ROW_BG
//     } else {
//         ALT_ROW_BG_COLOR
//     }
// }
