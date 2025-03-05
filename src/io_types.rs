pub mod dual;
pub mod single;

pub trait IOType: Copy + PartialEq {
    type Collection: IntoIterator<Item = Self>;
    fn all_combinations() -> Self::Collection;
}
