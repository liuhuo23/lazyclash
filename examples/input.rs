use color_eyre::{eyre::Ok, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout, Margin},
    style::{Color, Style, Stylize},
    symbols::scrollbar,
    text::{Line, Masked, Span},
    widgets::{block::Title, Block, Paragraph, StatefulWidget},
    DefaultTerminal, Frame,
};
use ratatui_input::{Input, InputState};
use std::{
    default,
    time::{Duration, Instant},
};
use tracing::debug;

#[derive(Default)]
enum AppState {
    Input,
    #[default]
    Normal,
}
#[derive(Default)]
struct App {
    input_state: InputState,
    app_state: AppState,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                let read_event = event::read()?;
                if let Event::Key(key) = read_event.clone() {
                    match self.app_state {
                        AppState::Normal => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('i') => self.app_state = AppState::Input,
                            _ => {}
                        },
                        AppState::Input => match key.code {
                            KeyCode::Esc => {
                                self.app_state = AppState::Normal;
                            }
                            _ => {
                                self.input_state.handle_message(read_event.into());
                            }
                        },
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::vertical(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);
        let mut title = Title::from("输入");
        match self.app_state {
            AppState::Input => title = Title::from("输入*"),
            _ => {}
        }
        let input = Input::default();
        let b = Block::bordered().title(title);
        let inner_area = b.inner(chunks[0]);
        frame.render_widget(b, chunks[0]);
        frame.render_stateful_widget(input, inner_area, &mut self.input_state);
        let b = Block::bordered().title("Message");
        let message = Paragraph::new(self.input_state.text().replace('\n', "|")).block(b);
        frame.render_widget(message, chunks[1]);
    }
}

#[cfg(test)]
mod test {
    enum Message {
        Cut,
        Copy,
        Delete,
    }
    enum Event {
        C,
        V,
        Backend,
    }
    impl From<Event> for Message {
        fn from(value: Event) -> Self {
            match value {
                Event::C => Message::Copy,
                Event::Backend => Message::Delete,
                Event::V => Message::Cut,
            }
        }
    }
    fn handle_message(message: Message) {
        println!("");
    }
    #[test]
    fn test_from() {
        let event = Event::C;
        // let message: Message = event.into();
        handle_message(event.into());
    }
}
