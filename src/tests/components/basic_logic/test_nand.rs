use crate::components::basic_logic::nand::NandGate;
use crate::components::Component;
use crate::io_types::dual::DualIO;
use crate::io_types::single::SingleIO;
use crate::truth_table::TruthTable;

// ToDo: Optimize tests so you can actually see whats wrong when something fails... lol
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
    assert!(gate.test_truth_table(truth_table));
}
