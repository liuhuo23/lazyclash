use crate::components::Component;

pub mod versions;

pub trait Menu: Component {
    fn get_length(&self) -> u16 {
        2
    }
    fn set_active(&mut self, active: bool);
    fn is_active(&self) -> bool;
    fn get_detail(&mut self) -> &mut Box<dyn Component>;
}

pub enum MenuActive {
    Version(Box<dyn Menu>),
}
