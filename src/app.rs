use crate::menus::VersionMenu;
use anyhow::{Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

pub enum MenuActive {
    Version(usize),
}

pub struct Menu {
    value: Box<dyn Function>,
}
impl Menu {
    pub fn new(t: Box<dyn Function>) -> Self {
        Self { value: t }
    }
}

impl Function for Menu {
    fn detail_draw(&mut self, react: ratatui::prelude::Rect) {
        self.value.detail_draw(react);
    }
    fn get_info(&self) -> String {
        self.value.get_info()
    }

    fn handle_event(&mut self) -> Result<()> {
        self.value.handle_event()
    }

    fn menu_draw(&mut self, react: ratatui::prelude::Rect) {
        self.value.menu_draw(react);
    }

    fn help_draw(&self, react: ratatui::prelude::Rect) {
        self.value.help_draw(react);
    }
}

use crate::functions::Function;
pub struct App {
    menus: Vec<Menu>,
    current_menu: Option<MenuActive>,
}

impl App {
    pub fn new() -> Self {
        let app = App {
            menus: vec![Menu::new(Box::new(VersionMenu::new()))],
            current_menu: Some(MenuActive::Version(0)),
        };
        app
    }
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
        // frame.render_widget(
        //     Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        //     content,
        // );
        let mut title = "".to_string();
        match &self.current_menu {
            Some(active) => {
                match active {
                    MenuActive::Version(index) => title = self.menus[*index].get_info(),
                    _ => {}
                };
            }
            None => {}
        };
        frame.render_widget(
            Paragraph::new(title).block(Block::new().borders(Borders::ALL)),
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
