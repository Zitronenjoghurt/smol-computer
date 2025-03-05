use crate::components::logic::nand::NandGate;
use crate::components::logic::not::NotGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AndGate {
    input: DualIO,
    nand: NandGate,
    not: NotGate,
    output: SingleIO,
}

impl Component for AndGate {
    type Input = DualIO;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        let nand_result = self.nand.process(self.input);
        self.output = self.not.process(nand_result);
        self.output()
    }

    fn update(&mut self, input: Self::Input) {
        self.input = input;
    }

    fn input(&self) -> Self::Input {
        self.nand.input()
    }

    fn output(&self) -> Self::Output {
        self.output
    }
}
