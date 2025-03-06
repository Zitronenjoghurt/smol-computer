use crate::io_types::single::SingleIO;
use crate::io_types::triple::TripleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SumCInIO(TripleIO);

impl SumCInIO {
    pub fn new(a: SingleIO, b: SingleIO, carry_in: SingleIO) -> Self {
        Self(TripleIO::new(a, b, carry_in))
    }

    pub fn a(&self) -> SingleIO {
        self.0.a()
    }

    pub fn b(&self) -> SingleIO {
        self.0.b()
    }

    pub fn carry_in(&self) -> SingleIO {
        self.0.c()
    }
}

impl IOType for SumCInIO {
    type Collection = Vec<SumCInIO>;

    fn all_combinations() -> Self::Collection {
        TripleIO::all_combinations()
            .into_iter()
            .map(SumCInIO)
            .collect()
    }
}
