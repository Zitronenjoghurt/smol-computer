use crate::components::basic_logic::nand::NandGate;
use crate::components::basic_logic::not::NotGate;
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
        self.output = self.nand.process((not_a, not_b).into());
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
