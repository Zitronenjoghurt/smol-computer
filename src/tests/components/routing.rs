use crate::assert_component_test;
use crate::components::routing::mux_2_to_1::{Mux2To1, Mux2To1Input};
use crate::io_types::single::SingleIO;
use crate::truth_table::TruthTable;

#[test]
fn test_mux_2_to_1() {
    let truth_table: TruthTable<Mux2To1Input, SingleIO> = TruthTable::create_from_values(vec![
        SingleIO::low(),
        SingleIO::low(),
        SingleIO::high(),
        SingleIO::low(),
        SingleIO::low(),
        SingleIO::high(),
        SingleIO::high(),
        SingleIO::high(),
    ])
    .unwrap();

    let mut component = Mux2To1::default();
    assert_component_test!(component, truth_table);
}
