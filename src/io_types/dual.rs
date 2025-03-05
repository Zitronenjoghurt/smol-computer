use crate::io_types::single::SingleIO;
use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DualIO {
    pub a: SingleIO,
    pub b: SingleIO,
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

impl From<(SingleIO, SingleIO)> for DualIO {
    fn from(value: (SingleIO, SingleIO)) -> Self {
        Self {
            a: value.0,
            b: value.1,
        }
    }
}
