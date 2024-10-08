use color_eyre::{eyre::Ok, Result};
use config::Config;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

/// 根据给定的url下载对应的订阅配置文件
pub fn download_yaml(url: &str, config: &Config) -> Result<()> {
    Ok(())
}
