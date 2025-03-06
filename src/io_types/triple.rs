use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TripleIO<T: IOType> {
    a: T,
    b: T,
    c: T,
}

impl<T: IOType> TripleIO<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }

    pub fn a(&self) -> T {
        self.a
    }

    pub fn b(&self) -> T {
        self.b
    }

    pub fn c(&self) -> T {
        self.c
    }
}

impl<T: IOType> IOType for TripleIO<T> {
    type Collection = Vec<TripleIO<T>>;

    fn all_combinations() -> Self::Collection {
        let a_combinations: Vec<T> = T::all_combinations().into_iter().collect();
        let b_combinations: Vec<T> = T::all_combinations().into_iter().collect();
        let c_combinations: Vec<T> = T::all_combinations().into_iter().collect();

        let mut result = Vec::new();
        for a in a_combinations.iter() {
            for b in b_combinations.iter() {
                for c in c_combinations.iter() {
                    result.push(TripleIO::new(*a, *b, *c));
                }
            }
        }

        result
    }
}
