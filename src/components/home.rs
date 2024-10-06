use std::collections::HashMap;

use super::Component;
use crate::{
    action::Action,
    config::{key_event_to_string, Config},
};
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, info};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Processing,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    pub show_help: bool,
    pub mode: Mode,
    pub input: Input,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub keymap: HashMap<KeyEvent, Action>,
    pub text: Vec<String>,
    pub last_events: Vec<KeyEvent>,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
        self.keymap = keymap;
        self
    }
    pub fn add(&mut self, s: String) {
        self.text.push(s);
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::ToggleShowHelp => self.show_help = !self.show_help,
            Action::CompleteInput(s) => self.add(s),
            Action::EnterNormal => {
                self.mode = Mode::Normal;
            }
            Action::EnterInsert => {
                self.mode = Mode::Insert;
            }
            Action::EnterProcessing => {
                self.mode = Mode::Processing;
            }
            Action::ExitProcessing => {
                self.mode = Mode::Normal;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let [left_menu, right_detail] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);
        frame.render_widget(Block::bordered().title("Left Menu"), left_menu);
        frame.render_widget(Block::bordered().title("right detail"), right_detail);
        // let rects = Layout::default()
        //     .constraints([Constraint::Percentage(100), Constraint::Min(3)].as_ref())
        //     .split(area);
        // let mut text: Vec<Line> = self
        //     .text
        //     .clone()
        //     .iter()
        //     .map(|l| Line::from(l.clone()))
        //     .collect();
        // text.insert(0, "".into());
        // text.insert(
        //     0,
        //     "Type into input and hit enter to display here".dim().into(),
        // );
        // text.insert(0, "".into());
        // text.insert(0, format!("Render Ticker: {}", self.render_ticker).into());
        // text.insert(0, format!("App Ticker: {}", self.app_ticker).into());
        // text.insert(0, "".into());
        // text.insert(
        //     0,
        //     Line::from(vec![
        //         "Press ".into(),
        //         Span::styled("j", Style::default().fg(Color::Red)),
        //         " or ".into(),
        //         Span::styled("k", Style::default().fg(Color::Red)),
        //         " to ".into(),
        //         Span::styled("increment", Style::default().fg(Color::Yellow)),
        //         " or ".into(),
        //         Span::styled("decrement", Style::default().fg(Color::Yellow)),
        //         ".".into(),
        //     ]),
        // );
        // text.insert(0, "".into());
        // frame.render_widget(
        //     Paragraph::new(text)
        //         .block(
        //             Block::default()
        //                 .title("ratatui async template")
        //                 .title_alignment(Alignment::Center)
        //                 .borders(Borders::ALL)
        //                 .border_style(match self.mode {
        //                     Mode::Processing => Style::default().fg(Color::Yellow),
        //                     _ => Style::default(),
        //                 })
        //                 .border_type(BorderType::Rounded),
        //         )
        //         .style(Style::default().fg(Color::Cyan))
        //         .alignment(Alignment::Center),
        //     rects[0],
        // );
        // let f = frame;
        // let width = rects[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor
        // let scroll = self.input.visual_scroll(width as usize);
        // let input = Paragraph::new(self.input.value())
        //     .style(match self.mode {
        //         Mode::Insert => Style::default().fg(Color::Yellow),
        //         _ => Style::default(),
        //     })
        //     .scroll((0, scroll as u16))
        //     .block(
        //         Block::default()
        //             .borders(Borders::ALL)
        //             .title(Line::from(vec![
        //                 Span::raw("Enter Input Mode "),
        //                 Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
        //                 Span::styled(
        //                     "/",
        //                     Style::default()
        //                         .add_modifier(Modifier::BOLD)
        //                         .fg(Color::Gray),
        //                 ),
        //                 Span::styled(" to start, ", Style::default().fg(Color::DarkGray)),
        //                 Span::styled(
        //                     "ESC",
        //                     Style::default()
        //                         .add_modifier(Modifier::BOLD)
        //                         .fg(Color::Gray),
        //                 ),
        //                 Span::styled(" to finish)", Style::default().fg(Color::DarkGray)),
        //             ])),
        //     );
        // f.render_widget(input, rects[1]);
        // if self.mode == Mode::Insert {
        //     let position = Position {
        //         x: (rects[1].x + 1 + self.input.cursor() as u16)
        //             .min(rects[1].x + rects[1].width - 2),
        //         y: rects[1].y + 1,
        //     };
        //     f.set_cursor_position(position)
        // }
        // let rect = area;
        // if self.show_help {
        //     let rect = rect.inner(Margin {
        //         horizontal: 4,
        //         vertical: 2,
        //     });
        //     f.render_widget(Clear, rect);
        //     let block = Block::default()
        //         .title(Line::from(vec![Span::styled(
        //             "Key Bindings",
        //             Style::default().add_modifier(Modifier::BOLD),
        //         )]))
        //         .borders(Borders::ALL)
        //         .border_style(Style::default().fg(Color::Yellow));
        //     f.render_widget(block, rect);
        //     let rows = vec![
        //         Row::new(vec!["j", "Increment"]),
        //         Row::new(vec!["k", "Decrement"]),
        //         Row::new(vec!["/", "Enter Input"]),
        //         Row::new(vec!["ESC", "Exit Input"]),
        //         Row::new(vec!["Enter", "Submit Input"]),
        //         Row::new(vec!["q", "Quit"]),
        //         Row::new(vec!["?", "Open Help"]),
        //     ];
        //     let table = Table::new(
        //         rows,
        //         [Constraint::Percentage(10), Constraint::Percentage(90)],
        //     )
        //     .header(
        //         Row::new(vec!["Key", "Action"])
        //             .bottom_margin(1)
        //             .style(Style::default().add_modifier(Modifier::BOLD)),
        //     )
        //     .column_spacing(1);
        //     f.render_widget(
        //         table,
        //         rect.inner(Margin {
        //             vertical: 4,
        //             horizontal: 2,
        //         }),
        //     );
        // };

        // f.render_widget(
        //     Block::default()
        //         .title(
        //             ratatui::widgets::block::Title::from(format!(
        //                 "{:?}",
        //                 &self
        //                     .last_events
        //                     .iter()
        //                     .map(key_event_to_string)
        //                     .collect::<Vec<_>>()
        //             ))
        //             .alignment(Alignment::Right),
        //         )
        //         .title_style(Style::default().add_modifier(Modifier::BOLD)),
        //     Rect {
        //         x: rect.x + 1,
        //         y: rect.height.saturating_sub(1),
        //         width: rect.width.saturating_sub(2),
        //         height: 1,
        //     },
        // );
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.last_events.push(key);
        let action = match self.mode {
            Mode::Normal | Mode::Processing => return Ok(None),
            Mode::Insert => match key.code {
                KeyCode::Esc => Action::EnterNormal,
                KeyCode::Enter => {
                    if let Some(sender) = &self.action_tx {
                        if let Err(e) =
                            sender.send(Action::CompleteInput(self.input.value().to_string()))
                        {
                            error!("Failed to send action: {:?}", e);
                        }
                    }
                    Action::EnterNormal
                }
                _ => {
                    self.input
                        .handle_event(&ratatui::crossterm::event::Event::Key(key));
                    Action::Update
                }
            },
        };
        Ok(Some(action))
    }
}
