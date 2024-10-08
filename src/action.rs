use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    ToggleShowHelp,
    CompleteInput(String),
    EnterNormal, // 正常模式
    EnterInsert, // 插入模式
    EnterProcessing,
    ExitProcessing,
    Update,
    EnterSubscribe,        // 触发订阅
    ExitSubscribe(String), // 退出订阅
    ClearScreen,
}
