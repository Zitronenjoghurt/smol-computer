use crate::io_types::single::SingleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TripleIO {
    a: SingleIO,
    b: SingleIO,
    c: SingleIO,
}

impl TripleIO {
    pub fn new(a: SingleIO, b: SingleIO, c: SingleIO) -> Self {
        Self { a, b, c }
    }

    pub fn a(&self) -> SingleIO {
        self.a
    }

    pub fn b(&self) -> SingleIO {
        self.b
    }

    pub fn c(&self) -> SingleIO {
        self.c
    }
}

impl IOType for TripleIO {
    type Collection = Vec<TripleIO>;

    fn all_combinations() -> Self::Collection {
        vec![
            TripleIO {
                a: SingleIO::low(),
                b: SingleIO::low(),
                c: SingleIO::low(),
            },
            TripleIO {
                a: SingleIO::low(),
                b: SingleIO::low(),
                c: SingleIO::high(),
            },
            TripleIO {
                a: SingleIO::low(),
                b: SingleIO::high(),
                c: SingleIO::low(),
            },
            TripleIO {
                a: SingleIO::low(),
                b: SingleIO::high(),
                c: SingleIO::high(),
            },
            TripleIO {
                a: SingleIO::high(),
                b: SingleIO::low(),
                c: SingleIO::low(),
            },
            TripleIO {
                a: SingleIO::high(),
                b: SingleIO::low(),
                c: SingleIO::high(),
            },
            TripleIO {
                a: SingleIO::high(),
                b: SingleIO::high(),
                c: SingleIO::low(),
            },
            TripleIO {
                a: SingleIO::high(),
                b: SingleIO::high(),
                c: SingleIO::high(),
            },
        ]
    }
}
