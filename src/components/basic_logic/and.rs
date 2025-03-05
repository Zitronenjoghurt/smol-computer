use crate::components::basic_logic::nand::NandGate;
use crate::components::basic_logic::not::NotGate;
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
        let nand_result = self.nand.evaluate();
        self.output = self.not.process(nand_result);
        self.output()
    }

    fn update(&mut self, input: Self::Input) {
        self.nand.update(input);
    }

    fn input(&self) -> Self::Input {
        self.nand.input()
    }

    fn output(&self) -> Self::Output {
        self.output
    }
}
