#![doc = include_str!("../README.md")]

mod axis;
mod bound;
mod common;
mod search;
mod table1d;

pub use axis::Axis;
pub use bound::{Bound, Clamp, Interp};
pub use search::{Binary, CachedLinearCell, Linear, RuntimeSearch, Search};
pub use table1d::LookupTable1D;

#[derive(Debug)]
pub enum Error {
    NonMonotonicSorting,
    DuplicateEntry,
    IndependentDependentLength,
    IndependentVariableTooShort,
}
