use std::default;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use futures::executor::block_on;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::{Block, Padding, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{
    action::Action,
    components::{fps::FpsCounter, home::Home, Component},
    config::Config,
    menus::{versions::Version, Menu, MenuActive},
    tui::{Event, Tui},
};

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    menu_active: usize,
    menus: Vec<Box<dyn Menu>>,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![Box::new(Home::new()), Box::new(FpsCounter::default())],
            menus: vec![
                Box::new(Version::new(true)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
                Box::new(Version::new(false)),
            ],
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            menu_active: 0,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    pub fn next(&mut self) {
        self.menu_active = (self.menu_active + 1) % self.menus.len();
        self.menus[self.menu_active].set_active(true);
        for i in 0..self.menus.len() {
            if i != self.menu_active {
                self.menus[i].set_active(false);
            }
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        // for component in self.components.iter_mut() {
        //     component.register_action_handler(self.action_tx.clone())?;
        // }
        // for component in self.components.iter_mut() {
        //     component.register_config_handler(self.config.clone())?;
        // }
        // for component in self.components.iter_mut() {
        //     component.init(tui.size()?)?;
        // }
        // 加载菜单
        for menu in self.menus.iter_mut() {
            menu.register_action_handler(self.action_tx.clone())?;
        }
        for menu in self.menus.iter_mut() {
            menu.register_config_handler(self.config.clone())?;
        }
        for menu in self.menus.iter_mut() {
            menu.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        for menu in self.menus.iter_mut() {
            if let Some(action) = menu.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        match key.code {
            KeyCode::Tab => {
                self.next();
            }
            _ => {
                let Some(keymap) = self.config.keybindings.get(&self.mode) else {
                    return Ok(());
                };
                match keymap.get(&vec![key]) {
                    Some(action) => {
                        info!("Got action: {action:?}");
                        action_tx.send(action.clone())?;
                    }
                    _ => {
                        // If the key was not handled as a single key action,
                        // then consider it for multi-key combinations.
                        self.last_tick_key_events.push(key);

                        // Check for multi-key combinations
                        if let Some(action) = keymap.get(&self.last_tick_key_events) {
                            info!("Got action: {action:?}");
                            action_tx.send(action.clone())?;
                        };
                    }
                }
            }
        };
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                _ => {}
            }
            // for component in self.components.iter_mut() {
            //     if let Some(action) = component.update(action.clone())? {
            //         self.action_tx.send(action)?
            //     };
            // }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let [left_menu, right_detail] =
                Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .areas(frame.area());
            let left_block = Block::bordered()
                .title("Left Menu")
                .padding(Padding::new(1, 1, 1, 1));
            let menus_lines = Layout::vertical(
                self.menus
                    .iter()
                    .map(|m| Constraint::Max(m.get_length()))
                    .collect::<Vec<Constraint>>(),
            )
            .split(left_menu);
            for (i, menu) in self.menus.iter_mut().enumerate() {
                if let Err(err) = menu.draw(frame, menus_lines[i]) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
                if menu.is_active() {
                    let detail = menu.get_detail();
                    if let Err(err) = detail.draw(frame, right_detail) {
                        let _ = self
                            .action_tx
                            .send(Action::Error(format!("Failed to draw: {:?}", err)));
                    }
                }
            }

            frame.render_widget(Block::bordered().title("right detail"), right_detail);
            frame.render_widget(left_block, left_menu);
        })?;
        Ok(())
    }
}
