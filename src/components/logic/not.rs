use crate::components::logic::nand::NandGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
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
        let nand_in = DualIO::new(self.input, self.input);
        self.output = self.nand.process(nand_in);
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
