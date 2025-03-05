use crate::io_types::IOType;

pub mod arithmetic;
pub mod logic;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum State {
    High,
    Low,
    #[default]
    Unknown,
}

pub trait Component: Sized {
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
}
