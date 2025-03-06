use std::fmt::Debug;

pub mod dual;
pub mod mux_in;
pub mod single;
pub mod sum_c_in;
pub mod sum_carry;
pub mod triple;

pub trait IOType: Copy + PartialEq + Debug {
    type Collection: IntoIterator<Item = Self>;
    fn all_combinations() -> Self::Collection;
}
