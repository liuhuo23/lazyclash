use anyhow::Result;
use ratatui::layout::Rect;
pub trait Function {
    fn get_info(self) -> String;
    fn handle_event(&mut self) -> Result<()>;
    fn menu_draw(&mut self, react: Rect);
    fn detail_draw(&mut self, react: Rect);
    fn help_draw(&self, react: Rect);
}
