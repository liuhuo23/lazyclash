use anyhow::Ok;

use crate::functions::Function;

pub struct VersionMenu {
    title: String,
    help: String,
}

impl VersionMenu {
    pub fn new() -> Self {
        Self {
            title: "Version".to_string(),
            help: "帮助信息".to_string(),
        }
    }
}

impl Function for VersionMenu {
    fn detail_draw(&mut self, react: ratatui::prelude::Rect) {
        todo!()
    }
    fn get_info(&self) -> String {
        self.help.clone()
    }
    fn handle_event(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn help_draw(&self, react: ratatui::prelude::Rect) {
        todo!()
    }
    fn menu_draw(&mut self, react: ratatui::prelude::Rect) {
        todo!()
    }
}
