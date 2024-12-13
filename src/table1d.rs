use crate::axis;
use crate::search;
use std::ops::{Add, Div, Mul, Sub};

struct LookupTable1D<Axis: axis::AxisImpl, Dep> {
    indep: Vec<<Axis as axis::AxisImpl>::Indep>,
    dep: Vec<Dep>,
    search: <Axis as axis::AxisImpl>::Search,
}

impl<Indep, Search, Dep> LookupTable1D<axis::Axis<Indep, Search>, Dep> {
    pub fn new(indep: Vec<Indep>, search: Search, dep: Vec<Dep>) -> Self {
        // TODO: length and monotonic checks
        Self {
            indep,
            search,
            dep
        }
    }
}

impl<Indep, Search, Dep> LookupTable1D<axis::Axis<Indep, Search>, Dep>
where
    Search: search::Search<Indep>,
    // TODO: HoldHigh / HoldLow does not require so many strict bounds
    Dep: Copy
        + Sub<Dep, Output = Dep>
        + Div<Indep, Output = Dep>
        + Mul<Indep, Output = Dep>
        + Add<Dep, Output = Dep>,
    Indep: Copy + Sub<Indep, Output = Indep> + std::cmp::PartialOrd,
{
    pub fn lookup(&self, x: &Indep) -> Dep {
        let (idx_l, idx_h) = self.search.search(x, self.indep.as_slice());

        let x_l: Indep = self.indep[idx_l];
        let x_h: Indep = self.indep[idx_h];

        let y_l: Dep = self.dep[idx_l];
        let y_h: Dep = self.dep[idx_h];

        let slope = (y_h - y_l) / (x_h - x_l);

        return slope * (*x - x_l) + y_l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TOL: f64 = 1e-10;

    fn linear_simple_table() -> LookupTable1D<axis::Axis<f64, search::Linear>, f64> {
        let x = vec![0., 1., 2., 3.];
        let y = vec![0., 1., 2., 3.];
        let search = search::Linear::default();
        LookupTable1D::new(x, search, y)
    }

    #[test]
    fn linear_1() {
        let table = linear_simple_table();
        let output = table.lookup(&0.5);
        float_eq::float_eq!(output, 0.5, abs <= TOL);
    }

    #[test]
    fn linear_lower_oob() {
        let table = linear_simple_table();
        let output = table.lookup(&-1.0);

        float_eq::float_eq!(output, 0.5, abs <= TOL);
    }

    #[test]
    fn linear_higher_oob() {
        let table = linear_simple_table();
        let output = table.lookup(&100.0);

        float_eq::float_eq!(output, 0.5, abs <= TOL);
    }
}
