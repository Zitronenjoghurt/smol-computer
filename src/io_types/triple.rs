use crate::io_types::single::SingleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TripleIO {
    pub a: SingleIO,
    pub b: SingleIO,
    pub c: SingleIO,
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

impl From<(SingleIO, SingleIO, SingleIO)> for TripleIO {
    fn from(value: (SingleIO, SingleIO, SingleIO)) -> Self {
        Self {
            a: value.0,
            b: value.1,
            c: value.2,
        }
    }
}
