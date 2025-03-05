use crate::io_types::IOType;

pub struct TruthTable<I: IOType, O: IOType> {
    pub entries: Vec<(I, O)>,
}

impl<I: IOType, O: IOType> TruthTable<I, O> {
    pub fn create_from_function(output_generator: impl Fn(I) -> O) -> TruthTable<I, O> {
        let inputs = I::all_combinations();
        let entries = inputs
            .into_iter()
            .map(|input| (input, output_generator(input)))
            .collect();

        TruthTable { entries }
    }

    pub fn create_from_values(expected_outputs: Vec<O>) -> Option<TruthTable<I, O>> {
        let inputs: Vec<I> = I::all_combinations().into_iter().collect();

        if inputs.len() == expected_outputs.len() {
            let entries = inputs.into_iter().zip(expected_outputs).collect();
            Some(TruthTable { entries })
        } else {
            None
        }
    }
}
