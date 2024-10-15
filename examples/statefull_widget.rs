use clipboard;
use clipboard::ClipboardProvider;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::*,
    widgets::{
        block::{BlockExt, Position, Title},
        Block, Borders, Paragraph, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};
use ratatui_input::{Input, InputState, Message};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    horization: u16,
    vertrail: u16,
    input_state: InputState,
}

fn message(event: Event) -> Message {
    match event {
        Event::FocusGained => Message::Focus,
        Event::FocusLost => Message::RemoveFocus,
        Event::Key(key) => key_message(key.clone()),
        Event::Mouse(_) => Message::Empty,
        Event::Paste(str) => Message::Paste(str),
        Event::Resize(_, _) => Message::Empty,
    }
}

fn key_message(value: KeyEvent) -> Message {
    if value.kind == KeyEventKind::Release {
        Message::Empty
    } else {
        match value.code {
            KeyCode::Backspace => Message::DeleteBeforeCursor,
            KeyCode::Enter => Message::RemoveFocus,
            KeyCode::Left => {
                if value.modifiers == KeyModifiers::SHIFT {
                    Message::MoveLeftWithSelection
                } else {
                    Message::MoveLeft
                }
            }
            KeyCode::Right => {
                if value.modifiers == KeyModifiers::SHIFT {
                    Message::MoveRightWithSelection
                } else {
                    Message::MoveRight
                }
            }
            KeyCode::Up => Message::Empty,
            KeyCode::Down => Message::Empty,
            KeyCode::Home => {
                if value.modifiers == KeyModifiers::SHIFT {
                    Message::JumpToStartWithSelection
                } else {
                    Message::JumpToStart
                }
            }
            KeyCode::End => {
                if value.modifiers == KeyModifiers::SHIFT {
                    Message::JumpToEndWithSelection
                } else {
                    Message::JumpToEnd
                }
            }
            KeyCode::PageUp => Message::Empty,
            KeyCode::PageDown => Message::Empty,
            KeyCode::Tab => Message::Char('\t'),
            KeyCode::BackTab => Message::Empty,
            KeyCode::Delete => Message::DeleteOnCursor,
            KeyCode::Insert => Message::ToggleInsertMode,
            KeyCode::F(_) => Message::Empty,
            KeyCode::Char(c) => match c {
                'c' => {
                    if value.modifiers == KeyModifiers::CONTROL {
                        Message::Copy
                    } else {
                        Message::Char('c')
                    }
                }
                'x' => {
                    if value.modifiers == KeyModifiers::CONTROL {
                        Message::Cut
                    } else {
                        Message::Char('x')
                    }
                }
                'v' => {
                    if value.modifiers == KeyModifiers::CONTROL {
                        match clipboard::ClipboardContext::new()
                            .and_then(|mut cc| cc.get_contents())
                        {
                            Ok(str) => Message::Paste(str),
                            Err(_) => Message::Empty,
                        }
                    } else {
                        Message::Char('v')
                    }
                }
                c => Message::Char(c),
            },
            KeyCode::Null => Message::Empty,
            KeyCode::Esc => Message::RemoveFocus,
            KeyCode::CapsLock => Message::Empty,
            KeyCode::ScrollLock => Message::Empty,
            KeyCode::NumLock => Message::Empty,
            KeyCode::PrintScreen => Message::Empty,
            KeyCode::Pause => Message::Empty,
            KeyCode::Menu => Message::Empty,
            KeyCode::KeypadBegin => Message::Empty,
            KeyCode::Media(_) => Message::Empty,
            KeyCode::Modifier(_) => Message::Empty,
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let chunk = Layout::vertical(vec![
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(10),
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(10),
        ])
        .split(frame.area());
        let b = Block::bordered()
            .title("中间")
            .title_alignment(Alignment::Center);
        let w = MyWidget::default().content("hello").block(b);
        frame.render_stateful_widget(w, frame.area(), &mut self.input_state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let read_event = event::read()?;
        let message = message(read_event.clone());
        if let event::Event::Key(key) = read_event.clone() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        self.exit = true;
                        return Ok(());
                    }
                    KeyCode::Up => {
                        self.vertrail = self.vertrail.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        self.vertrail = self.vertrail.saturating_add(1);
                    }
                    KeyCode::Esc => {}
                    _ => self.input_state.handle_message(message),
                };
            }
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Default)]
struct MyWidget<'a> {
    content: String,
    block: Option<Block<'a>>,
    input: Input,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Input,
    Normal,
}

impl<'a> MyWidget<'a> {
    fn new() -> Self {
        Self::default()
    }
    fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Widget for MyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = self.block.inner_if_some(area);
        buf.set_string(
            inner.x,
            inner.y,
            self.content.clone(),
            Style::default().fg(Color::Red),
        );
        if let Some(b) = self.block {
            b.render(area, buf);
        }
    }
}

impl<'a> StatefulWidget for MyWidget<'a> {
    type State = InputState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let inner = self.block.inner_if_some(area);
        let chunks = Layout::vertical(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner);

        buf.set_string(
            chunks[1].x,
            chunks[1].y,
            self.content.clone(),
            Style::default().fg(Color::Red),
        );
        if let Some(b) = self.block {
            b.render(area, buf);
        }
    }
}
