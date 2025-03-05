use crate::assert_component_test;
use crate::components::arithmetic::full_adder::FullAdder;
use crate::components::arithmetic::half_adder::HalfAdder;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;
use crate::io_types::sum_c_in::SumCInIO;
use crate::io_types::sum_carry::SumCarryIO;
use crate::truth_table::TruthTable;

#[test]
fn test_half_adder() {
    let truth_table: TruthTable<DualIO, SumCarryIO> = TruthTable::create_from_values(vec![
        SumCarryIO::new(SingleIO::low(), SingleIO::low()),
        SumCarryIO::new(SingleIO::high(), SingleIO::low()),
        SumCarryIO::new(SingleIO::high(), SingleIO::low()),
        SumCarryIO::new(SingleIO::low(), SingleIO::high()),
    ])
    .unwrap();

    let mut component = HalfAdder::default();
    assert_component_test!(component, truth_table);
}

#[test]
fn test_full_adder() {
    let truth_table: TruthTable<SumCInIO, SumCarryIO> = TruthTable::create_from_values(vec![
        SumCarryIO::new(SingleIO::low(), SingleIO::low()), // 0 0 0
        SumCarryIO::new(SingleIO::high(), SingleIO::low()), // 0 0 1
        SumCarryIO::new(SingleIO::high(), SingleIO::low()), // 0 1 0
        SumCarryIO::new(SingleIO::low(), SingleIO::high()), // 0 1 1
        SumCarryIO::new(SingleIO::high(), SingleIO::low()), // 1 0 0
        SumCarryIO::new(SingleIO::low(), SingleIO::high()), // 1 0 1
        SumCarryIO::new(SingleIO::low(), SingleIO::high()), // 1 1 0
        SumCarryIO::new(SingleIO::high(), SingleIO::high()), // 1 1 1
    ])
    .unwrap();

    let mut component = FullAdder::default();
    assert_component_test!(component, truth_table);
}
