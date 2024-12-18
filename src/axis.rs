use std::marker::PhantomData;

// todo: axis should specify the behavior at the bounds
// interp
// ZoH
// clamp

/// Defines an axis of an intedepdent variable, the behavior at the upper and lower bounds, and how
/// to search the axis.
///
/// Implements the [AxisImpl](crate::axis::AxisImpl) Trait
///
/// * `Indep` - type of the independent variable. Typically [`f32`] or [`f64`]
/// * `Search` - search method implementing the [Search](crate::Search) trait
/// * `LowerBound` - bounding behavior at the lower bound of the axis implementing the [Bound](crate::Bound) trait
/// * `UpperBound` - bounding behavior at the higher bound of the axis implementing the [Bound](crate::Bound) trait
pub struct Axis<Indep, Search, LowerBound, UpperBound> {
    _indep: PhantomData<Indep>,
    _search: PhantomData<Search>,
    _lower_bound: PhantomData<LowerBound>,
    _upper_bound: PhantomData<UpperBound>,
}

pub trait AxisImpl {
    type Indep;
    type Search;
    type LowerBound;
    type UpperBound;
}

impl<Indep, Search, LowerBound, UpperBound> AxisImpl
    for Axis<Indep, Search, LowerBound, UpperBound>
{
    type Indep = Indep;
    type Search = Search;
    type LowerBound = LowerBound;
    type UpperBound = UpperBound;
}
