use crate::details::version_detail::VersionDetail;
use crate::{action::Action, components::Component};
use color_eyre::eyre::Ok;
use color_eyre::eyre::Result;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use tokio::sync::mpsc::UnboundedSender;

use super::Menu;

#[derive(Default)]
pub struct Version {
    pub version: String,
    pub action_tx: Option<UnboundedSender<Action>>,
    pub detail_view: VersionDetail,
}

impl Version {
    pub fn new() -> Self {
        Self {
            version: "0.0.1".to_string(),
            detail_view: VersionDetail::new(),
            ..Default::default()
        }
    }
}

impl Component for Version {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        let block = Block::bordered().title("[version]");
        let version = Paragraph::new(self.version.clone()).block(block);
        frame.render_widget(version, area);
        Ok(())
    }
}

impl Menu for Version {
    fn get_length(&self) -> u16 {
        3
    }

    fn get_detail(&self) -> Box<dyn Component> {
        Box::new(self.detail_view.clone())
    }
}
