use crate::prfitem::PrfItem;

#[derive(Debug, Clone)]
pub enum Action {
    SubScription(String), // 订阅事件
    SubScriptionResult(PrfItem),
    Error(String),
    SubScriptionUpdate,
    UpdatePrfList(Vec<PrfItem>),
    SelectedItem(String),
}
