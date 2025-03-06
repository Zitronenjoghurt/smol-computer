use crate::io_types::dual::DualIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SumCarryIO<T: IOType>(DualIO<T>);

impl<T: IOType> SumCarryIO<T> {
    pub fn new(sum: T, carry: T) -> Self {
        Self(DualIO::new(sum, carry))
    }

    pub fn sum(&self) -> T {
        self.0.a()
    }

    pub fn carry(&self) -> T {
        self.0.b()
    }
}

impl<T: IOType> IOType for SumCarryIO<T> {
    type Collection = Vec<SumCarryIO<T>>;

    fn all_combinations() -> Self::Collection {
        DualIO::all_combinations()
            .into_iter()
            .map(SumCarryIO)
            .collect()
    }
}
