#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Version,
    Subscription,
}

impl From<i32> for Mode {
    fn from(value: i32) -> Self {
        match value {
            0 => Mode::Version,
            1 => Mode::Subscription,
            _ => Mode::Version,
        }
    }
}

impl Into<i32> for Mode {
    fn into(self) -> i32 {
        match self {
            Self::Version => 0,
            Self::Subscription => 1,
        }
    }
}
