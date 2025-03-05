use crate::components::basic_logic::nand::NandGate;
use crate::components::Component;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NotGate {
    input: SingleIO,
    nand: NandGate,
    output: SingleIO,
}

impl Component for NotGate {
    type Input = SingleIO;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        self.output = self.nand.process((self.input, self.input).into());
        self.output()
    }

    fn update(&mut self, input: Self::Input) {
        self.nand.update((input, input).into());
    }

    fn input(&self) -> Self::Input {
        self.input
    }

    fn output(&self) -> Self::Output {
        self.output
    }
}
