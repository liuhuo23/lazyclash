use crate::{
    action::Action,
    config::{get_subscribe_dir, Config},
    menu::{subscription::SubScription, version::Version},
    mode::Mode,
    prfitem::PrfItem,
    view::View,
};
use color_eyre::{Result, eyre::eyre};
use crossterm::event::{self, Event, KeyCode,};
use ratatui::crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::{Block, Paragraph},
    Frame,
};
use tracing::debug;

pub struct App {
    config: Config,
    should_quit: bool,
    menu_index: i32,
    mode: Mode,
    menus: Vec<Box<dyn View>>,
    info: String,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            config: Config::new()?,
            menu_index: 0,
            mode: Mode::Version,
            menus: vec![Box::new(Version::new()), Box::new(SubScription::new())],
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

    pub fn exit(&self) -> Result<()> {
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
            self.handle_events()?;
            // 获取当前获取焦点的事件
            let action = self.current_menus().get_events();
            self.handle_actions(action).await?;
        }
        debug!("程序退出");
        Ok(())
    }

    pub async fn handle_actions(&mut self, action: Option<Action>) -> Result<()> {
        if action.is_none() {
            return Ok(());
        }
        let res_action = match action.unwrap() {
            Action::SubScription(url) => {
                let res = PrfItem::from_url(&url).await;
                let action = match res {
                    Ok(item) => {
                        let filename = item.file.clone();
                        let file_data = item.file_data.clone();
                        let mut sub_dir = get_subscribe_dir();
                        if filename.is_none(){
                            return Err(eyre!("订阅文件名为空"));
                        }
                        sub_dir.push(format!("{}.yaml", filename.unwrap()));
                        if file_data.is_none(){
                            return Err(eyre!("订阅文件数据为空"));
                        }
                        debug!("订阅文件路径:{}", sub_dir.display());
                        if !sub_dir.exists(){
                            tokio::fs::create_dir_all(sub_dir.parent().unwrap()).await?;
                        }
                        tokio::fs::write(sub_dir, file_data.unwrap()).await?;
                        Action::SubScriptionResult(item)
                    }
                    Err(err) => Action::Error(format!("{err:?}")),
                };
                Some(action)
            }
            _ => None,
        };
        for menu in self.menus.iter_mut() {
            menu.update(res_action.clone())?;
        }
        Ok(())
    }

    fn current_menus(&mut self) -> &mut Box<dyn View> {
        &mut self.menus[self.menu_index as usize]
    }

    fn handle_events(&mut self) -> Result<()> {
        let mut event = Some(event::read()?);
        for menu in self.menus.iter_mut() {
            if menu.is_focus() {
                event = menu.handle_event(event.unwrap().clone());
            }
        }
        if let Some(Event::Key(key)) = event {
            if key.kind == event::KeyEventKind::Press {
                debug!("开始处理:{:?}", key.code);
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
            Layout::vertical([Constraint::Percentage(90), Constraint::Max(3)]).areas(f.area());
        let [left_panel, rigth_panel] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(main);

        self.draw_left(f, left_panel);
        self.draw_right(f, rigth_panel);
        self.draw_bottom_info(f, status);
    }

    fn draw_left(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::vertical(
            self.menus
                .iter()
                .map(|m| {
                    if m.is_focus() {
                        Constraint::Length(m.length())
                    } else {
                        Constraint::Length(5)
                    }
                })
                .collect::<Vec<Constraint>>(),
        )
        .split(area);
        for (i, menu) in self.menus.iter_mut().enumerate() {
            menu.draw_menu(f, chunks[i]);
        }
    }
    fn draw_right(&mut self, f: &mut Frame, area: Rect) {
        let b = Block::bordered();
        let inner_area = b.inner(area);
        let index: i32 = self.mode.into();
        f.render_widget(b, area);
        self.menus[index as usize].draw_detail(f, inner_area);
    }

    fn draw_bottom_info(&mut self, f: &mut Frame, area: Rect) {
        let p = Paragraph::new(format!(
            "当前name: {}, focus: {}",
            self.info.clone(),
            self.current_menus().is_focus()
        ))
        .block(Block::bordered());
        f.render_widget(p, area);
    }
}
