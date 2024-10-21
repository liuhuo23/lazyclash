use crate::{
    config::Config,
    menu::{subscription::SubScription, version::Version, Menu},
    mode::Mode,
};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use futures::SinkExt;
use ratatui::crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::{Block, Clear, Paragraph},
    Frame,
};
use tracing::{debug, info};

pub struct App {
    config: Config,
    should_quit: bool,
    menu_index: i32,
    mode: Mode,
    menus: Vec<Box<dyn Menu>>,
    info: String,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            config: Config::new()?,
            menu_index: 0,
            mode: Mode::Version,
            menus: vec![
                Box::new(Version::default()),
                Box::new(SubScription::default()),
            ],
            info: "提示信息".to_string(),
        })
    }

    pub fn next(&mut self) {
        if self.menu_index + 1 < self.menus.len() as i32 {
            self.menu_index += 1;
        } else {
            self.menu_index = 0;
        }
        self.mode = Mode::from(self.menu_index);
        self.set_focus();
        self.set_info(self.menus[self.menu_index as usize].name());
    }

    pub fn previous(&mut self) {
        if self.menu_index > 0 {
            self.menu_index -= 1;
        } else {
            self.menu_index = 0;
        }
        self.mode = Mode::from(self.menu_index);
        self.set_focus();
        self.set_info(self.menus[self.menu_index as usize].name());
    }

    fn set_info(&mut self, info: String) {
        self.info = info;
    }

    fn enter(&self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stderr(),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;
        Ok(())
    }

    fn exit(&self) -> Result<()> {
        crossterm::execute!(
            std::io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    fn set_focus(&mut self) {
        let index: i32 = self.mode.into();
        for (i, menu) in self.menus.iter_mut().enumerate() {
            if i as i32 == index && !menu.is_focus() {
                menu.set_focus();
                continue;
            }
            if menu.is_focus() {
                menu.set_focus();
            }
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        debug!("程序开始运行");
        self.enter()?;
        self.set_focus();
        let mut terminal = ratatui::init();
        while !self.should_quit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_events().await?;
        }
        self.exit()?;
        Ok(())
    }

    fn current_menus(&mut self) -> &mut Box<dyn Menu> {
        &mut self.menus[self.menu_index as usize]
    }

    async fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                        return Ok(());
                    }
                    KeyCode::Left => {
                        self.previous();
                    }
                    KeyCode::Right => {
                        self.next();
                    }
                    KeyCode::Tab => {
                        self.next();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let [main, status] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Max(10)]).areas(f.area());
        let [left_panel, rigth_panel] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(main);

        self.draw_left(f, left_panel);
        self.draw_right(f, rigth_panel);
        let p = Paragraph::new(format!(
            "当前name: {}, focus: {}",
            self.info.clone(),
            self.current_menus().is_focus()
        ));
        f.render_widget(p, status);
    }

    fn draw_left(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::vertical(
            self.menus
                .iter()
                .map(|_| Constraint::Length(10))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);
        for (i, menu) in self.menus.iter_mut().enumerate() {
            menu.draw_menu(f, chunks[i]);
        }
    }
    fn draw_right(&mut self, f: &mut Frame, area: Rect) {
        let index: i32 = self.mode.into();
        self.menus[index as usize].draw_detail(f, area);
    }
}
