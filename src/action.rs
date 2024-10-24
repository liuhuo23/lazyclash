use crate::prfitem::PrfItem;
pub enum Action {
    SubScription(String), // 订阅事件
    SubScriptionResult(PrfItem),
    Error(String),
}
