use crate::search;
use std::marker::PhantomData;

// todo: axis should specify the behavior at the bounds
// interp
// ZoH
// clamp

pub struct Axis<Indep, Search> {
    _indep: PhantomData<Indep>,
    _search: PhantomData<Search>,
}

// TODO: linear(), binary(), and cached() functions?
impl<Indep, Search> Axis<Indep, Search> {
    pub fn new() -> Self {
        Self {
            _indep: PhantomData,
            _search: PhantomData,
        }
    }
}

pub trait AxisImpl {
    type Indep;
    type Search;
}

impl<Indep, Search> AxisImpl for Axis<Indep, Search> {
    type Indep = Indep;
    type Search = Search;
}
