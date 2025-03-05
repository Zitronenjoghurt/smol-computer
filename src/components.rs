use crate::io_types::IOType;
use crate::truth_table::TruthTable;

pub mod basic_logic;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum State {
    High,
    Low,
    #[default]
    Unknown,
}

pub trait Component {
    type Input: IOType;
    type Output: IOType;
    fn evaluate(&mut self) -> Self::Output;
    fn update(&mut self, input: Self::Input);
    fn input(&self) -> Self::Input;
    fn output(&self) -> Self::Output;
    fn process(&mut self, input: Self::Input) -> Self::Output {
        self.update(input);
        self.evaluate()
    }
    // ToDo: Put in tests module
    fn test_io(&mut self, io: (Self::Input, Self::Output)) -> bool {
        self.process(io.0);
        self.output() == io.1
    }
    fn test_truth_table(&mut self, table: TruthTable<Self::Input, Self::Output>) -> bool {
        table.entries.iter().all(|io| self.test_io(*io))
    }
}
