use crate::components::logic::not::NotGate;
use crate::components::logic::or::OrGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NorGate {
    input: DualIO<SingleIO>,
    or: OrGate,
    not: NotGate,
    output: SingleIO,
}

impl Component for NorGate {
    type Input = DualIO<SingleIO>;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        let or_result = self.or.process(self.input);
        self.output = self.not.process(or_result);
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
