use crate::components::arithmetic::half_adder::HalfAdder;
use crate::components::logic::or::OrGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;
use crate::io_types::sum_c_in::SumCInIO;
use crate::io_types::sum_carry::SumCarryIO;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FullAdder {
    input: SumCInIO<SingleIO>,
    ha1: HalfAdder,
    ha2: HalfAdder,
    carry: OrGate,
    output: SumCarryIO<SingleIO>,
}

impl Component for FullAdder {
    type Input = SumCInIO<SingleIO>;
    type Output = SumCarryIO<SingleIO>;

    fn evaluate(&mut self) -> Self::Output {
        let h1_in = DualIO::new(self.input.a(), self.input.b());
        let ha1 = self.ha1.process(h1_in);

        let ha2_in = DualIO::new(self.input.carry_in(), ha1.sum());
        let ha2 = self.ha2.process(ha2_in);

        let carry_in = DualIO::new(ha1.carry(), ha2.carry());
        let carry = self.carry.process(carry_in);

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
