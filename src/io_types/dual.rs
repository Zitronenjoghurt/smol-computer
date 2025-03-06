use crate::io_types::single::SingleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DualIO {
    a: SingleIO,
    b: SingleIO,
}

impl DualIO {
    pub fn new(a: SingleIO, b: SingleIO) -> Self {
        Self { a, b }
    }

    pub fn a(&self) -> SingleIO {
        self.a
    }

    pub fn b(&self) -> SingleIO {
        self.b
    }
}

impl IOType for DualIO {
    type Collection = Vec<DualIO>;

    fn all_combinations() -> Self::Collection {
        vec![
            DualIO {
                a: SingleIO::low(),
                b: SingleIO::low(),
            },
            DualIO {
                a: SingleIO::low(),
                b: SingleIO::high(),
            },
            DualIO {
                a: SingleIO::high(),
                b: SingleIO::low(),
            },
            DualIO {
                a: SingleIO::high(),
                b: SingleIO::high(),
            },
        ]
    }
}
