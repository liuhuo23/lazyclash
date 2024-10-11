use crate::details::version_detail::VersionDetail;
use crate::{action::Action, components::Component};
use color_eyre::eyre::Ok;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug};

use super::Menu;

/// 这是用来标记谁来监听这个事件
#[derive(Default)]
enum Mode {
    #[default]
    Menu,
    Detail,
}

pub struct Subscribe {
    pub action_tx: Option<UnboundedSender<Action>>,
    pub detail_view: Option<Box<dyn Component>>,
    is_active: bool,
    last_events: Vec<KeyEvent>,
    mode: Mode,
}

impl Subscribe {
    pub fn new(is_active: bool) -> Self {
        Self {
            detail_view: Some(Box::new(VersionDetail::new())),
            is_active,
            action_tx: None,
            last_events: vec![],
            mode: Mode::default(),
        }
    }
}

impl Component for Subscribe {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let mut block = Block::bordered().title("[订阅]");
        if self.is_active {
            block = block.white();
        } else {
            block = block.gray();
        }
        frame.render_widget(block, area);
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.last_events.push(key);
        let action = match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Menu;
                Action::EnterNormal
            }
            KeyCode::Enter => {
                self.mode = Mode::Detail;
                Action::Update
            }
            KeyCode::Char('a') => {
                debug!("EnterSubscribe");
                Action::EnterSubscribe
            }
            _ => Action::Update,
        };
        Ok(Some(action))
    }
    fn is_active(&self) -> bool {
        self.is_active
    }
    fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Menu for Subscribe {
    fn get_length(&self) -> u16 {
        3
    }

    fn get_detail(&mut self) -> &mut Option<Box<dyn Component>> {
        &mut self.detail_view
    }

    
}
