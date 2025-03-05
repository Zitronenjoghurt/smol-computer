use crate::components::{Component, State};
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NandGate {
    input: DualIO,
    output: SingleIO,
}

impl Component for NandGate {
    type Input = DualIO;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        self.output = evaluate_nand(self.input.a.value, self.input.b.value).into();
        self.output()
    }

    fn update(&mut self, input: Self::Input) {
        self.input = input;
    }

    fn input(&self) -> Self::Input {
        self.input
    }

    fn output(&self) -> Self::Output {
        self.output
    }
}

/// This will be the only time we use high level comparison operations
fn evaluate_nand(a: State, b: State) -> State {
    if a == State::Unknown || b == State::Unknown {
        State::Unknown
    } else if a == State::High && b == State::High {
        State::Low
    } else {
        State::High
    }
}
