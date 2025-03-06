use crate::components::logic::nand::NandGate;
use crate::components::logic::not::NotGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct OrGate {
    input: DualIO,
    not_a: NotGate,
    not_b: NotGate,
    nand: NandGate,
    output: SingleIO,
}

impl Component for OrGate {
    type Input = DualIO;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        let not_a = self.not_a.process(self.input.a());
        let not_b = self.not_b.process(self.input.b());

        let nand_in = DualIO::new(not_a, not_b);
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
