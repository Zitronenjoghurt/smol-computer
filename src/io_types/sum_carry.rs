use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SumCarryIO(DualIO);

impl SumCarryIO {
    pub fn new(sum: SingleIO, carry: SingleIO) -> Self {
        Self(DualIO::new(sum, carry))
    }

    pub fn sum(&self) -> SingleIO {
        self.0.a()
    }

    pub fn carry(&self) -> SingleIO {
        self.0.b()
    }
}

impl IOType for SumCarryIO {
    type Collection = Vec<SumCarryIO>;

    fn all_combinations() -> Self::Collection {
        DualIO::all_combinations()
            .into_iter()
            .map(SumCarryIO)
            .collect()
    }
}
