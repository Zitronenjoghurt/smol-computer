use crate::components::logic::nand::NandGate;
use crate::components::logic::not::NotGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::mux_in::MuxInIO;
use crate::io_types::single::SingleIO;

pub type Mux2To1Input = MuxInIO<DualIO<SingleIO>, SingleIO>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mux2To1 {
    input: Mux2To1Input,
    not_select: NotGate,
    nand1: NandGate,
    nand2: NandGate,
    nand3: NandGate,
    output: SingleIO,
}

impl Component for Mux2To1 {
    type Input = Mux2To1Input;
    type Output = SingleIO;

    fn evaluate(&mut self) -> Self::Output {
        let not_select = self.not_select.process(self.input.select());

        let nand1_in = DualIO::new(self.input.input().a(), self.input.select());
        let nand1 = self.nand1.process(nand1_in);

        let nand2_in = DualIO::new(self.input.input().b(), not_select);
        let nand2 = self.nand2.process(nand2_in);

        let nand3_in = DualIO::new(nand1, nand2);
        self.output = self.nand3.process(nand3_in);

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
