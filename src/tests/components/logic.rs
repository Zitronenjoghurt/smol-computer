use crate::assert_component_test;
use crate::components::logic::and::AndGate;
use crate::components::logic::nand::NandGate;
use crate::components::logic::nor::NorGate;
use crate::components::logic::not::NotGate;
use crate::components::logic::or::OrGate;
use crate::components::logic::xor::XorGate;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;
use crate::truth_table::TruthTable;

#[test]
fn test_nand() {
    let truth_table: TruthTable<DualIO, SingleIO> = TruthTable::create_from_values(vec![
        SingleIO::high(),
        SingleIO::high(),
        SingleIO::high(),
        SingleIO::low(),
    ])
    .unwrap();

    let mut gate = NandGate::default();
    assert_component_test!(gate, truth_table);
}

#[test]
fn test_and() {
    let truth_table: TruthTable<DualIO, SingleIO> = TruthTable::create_from_values(vec![
        SingleIO::low(),
        SingleIO::low(),
        SingleIO::low(),
        SingleIO::high(),
    ])
    .unwrap();

    let mut gate = AndGate::default();
    assert_component_test!(gate, truth_table);
}

#[test]
fn test_nor() {
    let truth_table: TruthTable<DualIO, SingleIO> = TruthTable::create_from_values(vec![
        SingleIO::high(),
        SingleIO::low(),
        SingleIO::low(),
        SingleIO::low(),
    ])
    .unwrap();

    let mut gate = NorGate::default();
    assert_component_test!(gate, truth_table);
}

#[test]
fn test_not() {
    let truth_table: TruthTable<SingleIO, SingleIO> =
        TruthTable::create_from_values(vec![SingleIO::high(), SingleIO::low()]).unwrap();

    let mut gate = NotGate::default();
    assert_component_test!(gate, truth_table);
}

#[test]
fn test_or() {
    let truth_table: TruthTable<DualIO, SingleIO> = TruthTable::create_from_values(vec![
        SingleIO::low(),
        SingleIO::high(),
        SingleIO::high(),
        SingleIO::high(),
    ])
    .unwrap();

    let mut gate = OrGate::default();
    assert_component_test!(gate, truth_table);
}

#[test]
fn test_xor() {
    let truth_table: TruthTable<DualIO, SingleIO> = TruthTable::create_from_values(vec![
        SingleIO::low(),
        SingleIO::high(),
        SingleIO::high(),
        SingleIO::low(),
    ])
    .unwrap();

    let mut gate = XorGate::default();
    assert_component_test!(gate, truth_table);
}
