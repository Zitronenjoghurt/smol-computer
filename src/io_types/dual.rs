use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DualIO<T: IOType> {
    a: T,
    b: T,
}

impl<T: IOType> DualIO<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    pub fn a(&self) -> T {
        self.a
    }

    pub fn b(&self) -> T {
        self.b
    }
}

impl<T: IOType> IOType for DualIO<T> {
    type Collection = Vec<DualIO<T>>;

    fn all_combinations() -> Self::Collection {
        let a_combinations: Vec<T> = T::all_combinations().into_iter().collect();
        let b_combinations: Vec<T> = T::all_combinations().into_iter().collect();

        let mut result = Vec::new();
        for a in a_combinations.iter() {
            for b in b_combinations.iter() {
                result.push(DualIO::new(*a, *b));
            }
        }

        result
    }
}
