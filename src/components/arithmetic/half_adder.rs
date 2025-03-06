use crate::components::logic::and::AndGate;
use crate::components::logic::xor::XorGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;
use crate::io_types::sum_carry::SumCarryIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HalfAdder {
    input: DualIO<SingleIO>,
    sum: XorGate,
    carry: AndGate,
    output: SumCarryIO<SingleIO>,
}

impl Component for HalfAdder {
    type Input = DualIO<SingleIO>;
    type Output = SumCarryIO<SingleIO>;

    fn evaluate(&mut self) -> Self::Output {
        let sum = self.sum.process(self.input);
        let carry = self.carry.process(self.input);
        self.output = SumCarryIO::new(sum, carry);
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
