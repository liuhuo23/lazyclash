use crate::components::Component;

pub mod versions;

pub trait Menu: Component {
    fn get_length(&self) -> u16 {
        2
    }

    fn get_detail(&self) -> Box<dyn Component>;
}
