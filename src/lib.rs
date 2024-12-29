#![doc = include_str!("../README.md")]

mod axis;
mod bound;
mod common;
mod search;
mod table1d;

#[cfg(feature = "ndarray")]
mod table2d;
#[cfg(all(feature = "ndarray", feature="num-traits"))]
mod table3d;

pub use axis::{Axis, AxisImpl};
pub use bound::{Bound, Clamp, Interp};
pub use search::{Binary, CachedLinearCell, Linear, RuntimeSearch, Search};
pub use table1d::LookupTable1D;

#[cfg(feature = "ndarray")]
pub use table2d::LookupTable2D;
#[cfg(all(feature = "ndarray", feature="num-traits"))]
pub use table3d::LookupTable3D;

/// Possible errors occuring at table construction
#[derive(Debug)]
pub enum Error {
    /// The provided data on at least one axis was not strictly monotonically increasing or
    /// strictly monotonically decreasing.
    NonMonotonicSorting,
    /// An entry in an independent variable axis was present more than once. This implies the data is not
    /// strictly monotonic.
    DuplicateEntry,
    /// The length of an independent variable axis did not match the corresponding dependent
    /// variable length.
    IndependentDependentLength,
    /// The independent variable provided had a length less than two
    IndependentVariableTooShort,
}
