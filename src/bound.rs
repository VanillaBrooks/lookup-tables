pub struct Interp;

pub struct Clamp;

trait Bound<Indep> {
    fn upper_bound(indep: Indep, upper_bound: Indep) -> Indep;
    fn lower_bound(indep: Indep, lower_bound: Indep) -> Indep;
}

impl<Indep> Bound<Indep> for Clamp
where
    Indep: std::cmp::PartialOrd,
{
    fn upper_bound(indep: Indep, upper_bound: Indep) -> Indep {
        // dont use std::cmp::max here because it requires Ord, which floats dont have
        if indep > upper_bound {
            upper_bound
        } else {
            indep
        }
    }

    fn lower_bound(indep: Indep, lower_bound: Indep) -> Indep {
        // dont use std::cmp::min here because it requires Ord, which floats dont have
        if indep < lower_bound {
            lower_bound
        } else {
            indep
        }
    }
}

impl<Indep> Bound<Indep> for Interp {
    fn upper_bound(indep: Indep, _upper_bound: Indep) -> Indep {
        indep
    }

    fn lower_bound(indep: Indep, _lower_bound: Indep) -> Indep {
        indep
    }
}
