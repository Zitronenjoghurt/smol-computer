use crate::components::State;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SingleIO {
    pub value: State,
}

impl SingleIO {
    pub fn high() -> Self {
        Self { value: State::High }
    }

    pub fn low() -> Self {
        Self { value: State::Low }
    }
}

impl IOType for SingleIO {
    type Collection = Vec<SingleIO>;

    fn all_combinations() -> Self::Collection {
        vec![
            SingleIO { value: State::Low },
            SingleIO { value: State::High },
        ]
    }
}

impl From<State> for SingleIO {
    fn from(value: State) -> Self {
        SingleIO { value }
    }
}
