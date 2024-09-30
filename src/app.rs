use anyhow::{Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
#[derive(Default, Debug)]
pub struct App {}

impl App {
    /// TUI 入口函数
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                };
            }
        }
    }

    /// UI 绘制
    fn draw(&self, frame: &mut Frame) {
        let [content, info] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Min(3)])
            .areas(frame.area());
        frame.render_widget(
            Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
            content,
        );
        frame.render_widget(
            Paragraph::new("Bottom").block(Block::new().borders(Borders::ALL)),
            info,
        );
        let [menu_layout, detail_layout] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .areas(content);
        frame.render_widget(
            Paragraph::new("inner 0").block(Block::new().borders(Borders::ALL)),
            menu_layout,
        );
        frame.render_widget(
            Paragraph::new("inner 1").block(Block::new().borders(Borders::ALL)),
            detail_layout,
        );
    }
}
