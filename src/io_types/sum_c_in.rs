use crate::io_types::triple::TripleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SumCInIO<T: IOType>(TripleIO<T>);

impl<T: IOType> SumCInIO<T> {
    pub fn new(a: T, b: T, carry_in: T) -> Self {
        Self(TripleIO::new(a, b, carry_in))
    }

    pub fn a(&self) -> T {
        self.0.a()
    }

    pub fn b(&self) -> T {
        self.0.b()
    }

    pub fn carry_in(&self) -> T {
        self.0.c()
    }
}

impl<T: IOType> IOType for SumCInIO<T> {
    type Collection = Vec<SumCInIO<T>>;

    fn all_combinations() -> Self::Collection {
        TripleIO::all_combinations()
            .into_iter()
            .map(SumCInIO)
            .collect()
    }
}
