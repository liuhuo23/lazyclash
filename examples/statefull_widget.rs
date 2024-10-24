use crossterm::event::{self, KeyCode,  KeyEventKind,};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{BlockExt, Position, Title},
        Block,  Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use ratatui_input::InputState;

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
    vertrail: u16,
    input_state: InputState,
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
        let b = Block::bordered().title("中间").title_alignment(Alignment::Center);
        let w = MyWidget::default().content("hello").block(b);
        frame.render_stateful_widget(w, frame.area(), &mut self.input_state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        let read_event = event::read()?;
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
                    _ => self.input_state.handle_message(read_event.into()),
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
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))
            .border_set(border::THICK);

        let counter_text =
            Text::from(vec![Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()])]);

        Paragraph::new(counter_text).centered().block(block).render(area, buf);
    }
}

#[derive(Default)]
struct MyWidget<'a> {
    content: String,
    block: Option<Block<'a>>,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Normal,
}

impl<'a> MyWidget<'a> {
    
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
        buf.set_string(inner.x, inner.y, self.content.clone(), Style::default().fg(Color::Red));
        if let Some(b) = self.block {
            b.render(area, buf);
        }
    }
}

impl<'a> StatefulWidget for MyWidget<'a> {
    type State = InputState;
    fn render(self, area: Rect, buf: &mut Buffer, _: &mut Self::State) {
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
