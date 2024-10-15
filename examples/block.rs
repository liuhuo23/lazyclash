use std::io;

use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};

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

    fn draw(&self, frame: &mut Frame) {
        let chunk = Layout::vertical(vec![
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(10),
            Constraint::Max(10),
            Constraint::Max(10),
            Constraint::Min(10),
        ])
        .split(frame.area());

        let b = Block::default().borders(Borders::ALL);
        let p = Paragraph::new(
            r#"
非常长的一段文字非常长的一段文字非常长的            非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长到这儿结束
            常长的一段文字"#,
        )
        .style(Style::default().fg(Color::Red))
        .block(b);
        frame.render_widget(p, chunk[0]);
        let title = Title::from(Line::from(vec!["Right".into()]));
        let b = Block::default()
            .borders(Borders::ALL)
            .title(title.alignment(Alignment::Right));
        let p = Paragraph::new(
            r#"
非常长的一段文字非常长的一段文字非常长的            非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长到这儿结束
            常长的一段文字"#,
        )
        .style(Style::default().fg(Color::Red))
        .wrap(Wrap::default())
        .block(b);
        frame.render_widget(p, chunk[1]);
        let b = Block::bordered().title(Title::from("left").alignment(Alignment::Left));
        let p = Paragraph::new(
            r#"
非常长的一段文字非常长的一段文字非常长的            非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长的非常长的一段文字非常长的一段文字非常长到这儿结束
            常长的一段文字"#,
        )
        .style(Style::default().fg(Color::Red))
        // (x, y)
        .scroll((self.vertrail, self.horization))
        .block(b);
        frame.render_widget(p, chunk[2]);
        let b = Block::default()
            .title(Title::from("Left Title").alignment(Alignment::Left))
            .title(Title::from("Middle Title").alignment(Alignment::Center))
            .title(Title::from("Right Title").alignment(Alignment::Right))
            .borders(Borders::ALL);
        let w = MyWidget {
            content: "自定义".to_string(),
        };
        frame.render_widget(w, chunk[3]);
        // frame.render_widget(b, chunk[3]);
        let b = Block::bordered()
            .title("Styled Header")
            .border_style(Style::default().fg(Color::Red));
        frame.render_widget(b, chunk[4]);
        //---外---------------
        //|  内              |
        //| |              | |
        //|                 |
        //--------------------
        let b = Block::bordered().title("外部");
        let inner_block = Block::bordered().title("内部");
        let inner_area = b.inner(chunk[5]);
        frame.render_widget(b, chunk[5]);
        frame.render_widget(inner_block, inner_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
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

                    _ => {}
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

struct MyWidget {
    content: String,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        buf.set_string(
            area.left(),
            area.top(),
            &self.content,
            Style::default().fg(Color::Yellow),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
