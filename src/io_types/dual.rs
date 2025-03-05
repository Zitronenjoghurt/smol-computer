use crate::components::State;
use crate::io_types::single::SingleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DualIO {
    pub a: State,
    pub b: State,
}

impl DualIO {
    pub fn a(&self) -> SingleIO {
        self.a.into()
    }

    pub fn b(&self) -> SingleIO {
        self.b.into()
    }
}

impl IOType for DualIO {
    type Collection = Vec<DualIO>;

    fn all_combinations() -> Self::Collection {
        vec![
            DualIO {
                a: State::Low,
                b: State::Low,
            },
            DualIO {
                a: State::Low,
                b: State::High,
            },
            DualIO {
                a: State::High,
                b: State::Low,
            },
            DualIO {
                a: State::High,
                b: State::High,
            },
        ]
    }
}

impl From<(SingleIO, SingleIO)> for DualIO {
    fn from(value: (SingleIO, SingleIO)) -> Self {
        Self {
            a: value.0.value,
            b: value.1.value,
        }
    }
}
