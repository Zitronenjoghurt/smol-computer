use crate::components::logic::not::NotGate;
use crate::components::logic::xor::XorGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct XnorGate {
    input: DualIO,
    xor: XorGate,
    not: NotGate,
    output: SingleIO,
}

impl Component for XnorGate {
    type Input = DualIO;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        let xor_result = self.xor.process(self.input);
        self.output = self.not.process(xor_result);
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
