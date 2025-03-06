use crate::io_types::IOType;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MuxInIO<I: IOType, S: IOType> {
    input: I,
    select: S,
}

impl<I: IOType, S: IOType> MuxInIO<I, S> {
    pub fn new(input: I, select: S) -> Self {
        Self { input, select }
    }

    pub fn input(&self) -> I {
        self.input
    }

    pub fn select(&self) -> S {
        self.select
    }
}

impl<I: IOType, S: IOType> IOType for MuxInIO<I, S> {
    type Collection = Vec<MuxInIO<I, S>>;

    fn all_combinations() -> Self::Collection {
        let input_combinations = I::all_combinations().into_iter().collect::<Vec<_>>();
        let select_combinations = S::all_combinations().into_iter().collect::<Vec<_>>();

        let mut result = Vec::new();
        for input in input_combinations.iter() {
            for select in select_combinations.iter() {
                result.push(MuxInIO::new(*input, *select));
            }
        }

        result
    }
}
