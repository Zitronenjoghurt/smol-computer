use crate::components::arithmetic::half_adder::HalfAdder;
use crate::components::logic::or::OrGate;
use crate::components::Component;
use crate::io_types::sum_c_in::SumCInIO;
use crate::io_types::sum_carry::SumCarryIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FullAdder {
    input: SumCInIO,
    ha1: HalfAdder,
    ha2: HalfAdder,
    carry: OrGate,
    output: SumCarryIO,
}

impl Component for FullAdder {
    type Input = SumCInIO;
    type Output = SumCarryIO;

    fn evaluate(&mut self) -> Self::Output {
        let ha1 = self.ha1.process((self.input.a(), self.input.b()).into());
        let ha2 = self.ha2.process((self.input.carry_in(), ha1.sum()).into());
        let carry = self.carry.process((ha1.carry(), ha2.carry()).into());
        self.output = SumCarryIO::new(ha2.sum(), carry);
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
