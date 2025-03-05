use crate::components::logic::nand::NandGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct XorGate {
    input: DualIO,
    nand1: NandGate, // A NAND B
    nand2: NandGate, // A NAND (A NAND B)
    nand3: NandGate, // B NAND (A NAND B)
    nand4: NandGate, // nand2 NAND nand3
    output: SingleIO,
}

impl Component for XorGate {
    type Input = DualIO;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        let nand_1 = self.nand1.process(self.input);
        let nand_2 = self.nand2.process((self.input.a(), nand_1).into());
        let nand_3 = self.nand3.process((self.input.b(), nand_1).into());
        self.output = self.nand4.process((nand_2, nand_3).into());
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
