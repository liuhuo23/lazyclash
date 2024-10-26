use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};
pub mod fps;

use crate::action::Action;
pub trait View {
    /// 绘制菜单
    fn draw_menu(&mut self, f: &mut Frame, area: Rect);
    /// 绘制详情页
    fn draw_detail(&mut self, f: &mut Frame, area: Rect);
    /// 处理事件
    fn handle_event(&mut self, event: Event) -> Option<Event> {
        Some(event)
    }
    /// 是否获得焦点
    fn is_focus(&self) -> bool {
        false
    }
    /// 失去或获得焦点
    fn set_focus(&mut self);
    /// 菜单名称
    fn name(&self) -> String;
    /// 获取长度
    fn length(&self) -> u16;
    /// 获取事件流
    fn get_events(&mut self) -> Option<Action> {
        None
    }
    /// 更新数据
    fn update(&mut self, action: Option<Action>) -> Result<()> {
        let _ = action;
        Ok(())
    }
}
