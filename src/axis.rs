use std::marker::PhantomData;

// todo: axis should specify the behavior at the bounds
// interp
// ZoH
// clamp

pub struct Axis<Indep, Search, LowerBound, UpperBound> {
    _indep: PhantomData<Indep>,
    _search: PhantomData<Search>,
    _lower_bound: PhantomData<LowerBound>,
    _upper_bound: PhantomData<UpperBound>,
}

// TODO: linear(), binary(), and cached() functions?
impl<Indep, Search, LowerBound, UpperBound> Axis<Indep, Search, LowerBound, UpperBound> {
    pub fn new() -> Self {
        Self {
            _indep: PhantomData,
            _search: PhantomData,
            _lower_bound: PhantomData,
            _upper_bound: PhantomData,
        }
    }
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
