use crate::axis;
use crate::bound;
use crate::common;
use crate::search;
use crate::Error;
use std::ops::{Add, Div, Mul, Sub};

use ndarray::Array2;

pub struct LookupTable2D<Axis1, Axis2, Dep>
where
    Axis1: axis::AxisImpl,
    Axis2: axis::AxisImpl,
{
    indep1: Vec<<Axis1 as axis::AxisImpl>::Indep>,
    search1: <Axis1 as axis::AxisImpl>::Search,
    indep2: Vec<<Axis2 as axis::AxisImpl>::Indep>,
    search2: <Axis2 as axis::AxisImpl>::Search,
    dep: Array2<Dep>,
}

impl<Indep1, Search1, LowerBound1, UpperBound1, Indep2, Search2, LowerBound2, UpperBound2, Dep>
    LookupTable2D<
        axis::Axis<Indep1, Search1, LowerBound1, UpperBound1>,
        axis::Axis<Indep2, Search2, LowerBound2, UpperBound2>,
        Dep,
    >
where
    Indep1: std::cmp::PartialOrd,
    Indep2: std::cmp::PartialOrd,
{
    pub fn new(
        mut indep1: Vec<Indep1>,
        search1: Search1,
        mut indep2: Vec<Indep2>,
        search2: Search2,
        mut dep: Array2<Dep>,
    ) -> Result<Self, Error> {
        match common::check_independent_variable(indep1.as_slice())? {
            common::IndependentVariableOrdering::MonotonicallyIncreasing => {}
            common::IndependentVariableOrdering::MonotonicallyDecreasing => {
                indep1.reverse();
                dep.invert_axis(ndarray::Axis(0));
                dbg!("reversing");
            }
        }

        match common::check_independent_variable(indep2.as_slice())? {
            common::IndependentVariableOrdering::MonotonicallyIncreasing => {}
            common::IndependentVariableOrdering::MonotonicallyDecreasing => {
                indep2.reverse();
                dep.invert_axis(ndarray::Axis(1));
                dbg!("reversing");
            }
        }

        common::check_lengths(indep1.len(), dep.len_of(ndarray::Axis(0)))?;
        common::check_lengths(indep2.len(), dep.len_of(ndarray::Axis(1)))?;

        Ok(Self {
            indep1,
            search1,
            indep2,
            search2,
            dep,
        })
    }
}

impl<Indep1, Search1, LowerBound1, UpperBound1, Indep2, Search2, LowerBound2, UpperBound2, Dep>
    LookupTable2D<
        axis::Axis<Indep1, Search1, LowerBound1, UpperBound1>,
        axis::Axis<Indep2, Search2, LowerBound2, UpperBound2>,
        Dep,
    >
where
    Search1: search::Search<Indep1>,
    Search2: search::Search<Indep2>,
    // TODO: HoldHigh / HoldLow does not require so many strict bounds
    Dep: Copy
        + Sub<Dep, Output = Dep>
        + Div<Indep1, Output = Dep>
        + Mul<Indep1, Output = Dep>
        + Mul<Indep2, Output = Dep>
        + Add<Dep, Output = Dep>
        + std::fmt::Debug,
    Indep1: Copy
        + Sub<Indep1, Output = Indep1>
        + std::cmp::PartialOrd
        + Div<Indep1, Output = Indep1>
        //
        + std::fmt::Debug,
    Indep2: Copy
        + Sub<Indep2, Output = Indep2>
        + std::cmp::PartialOrd
        + Div<Indep2, Output = Indep2>
        //
        + std::fmt::Debug,
    LowerBound1: bound::Bound<Indep1>,
    UpperBound1: bound::Bound<Indep1>,
    LowerBound2: bound::Bound<Indep2>,
    UpperBound2: bound::Bound<Indep2>,
{
    pub fn lookup(&self, x: Indep1, y: Indep2) -> Dep {
        let (idx_x_1, idx_x_2) = self.search1.search(x, self.indep1.as_slice());
        let (idx_y_1, idx_y_2) = self.search2.search(y, self.indep2.as_slice());

        let x_1: Indep1 = self.indep1[idx_x_1];
        let x_2: Indep1 = self.indep1[idx_x_2];

        let y_1: Indep2 = self.indep2[idx_y_1];
        let y_2: Indep2 = self.indep2[idx_y_2];

        let f_1_1: Dep = self.dep[[idx_x_1, idx_y_1]];
        let f_1_2: Dep = self.dep[[idx_x_1, idx_y_2]];
        let f_2_1: Dep = self.dep[[idx_x_2, idx_y_1]];
        let f_2_2: Dep = self.dep[[idx_x_2, idx_y_2]];

        // bound x acording to the axis we are interpolating on
        // unwrap is safe here as we have checked its at least length 2
        let x = LowerBound1::lower_bound(x, *self.indep1.first().unwrap());
        let x = UpperBound1::upper_bound(x, *self.indep1.last().unwrap());

        // bound y acording to the axis we are interpolating on
        // unwrap is safe here as we have checked its at least length 2
        let y = LowerBound2::lower_bound(y, *self.indep2.first().unwrap());
        let y = UpperBound2::upper_bound(y, *self.indep2.last().unwrap());

        let x_slope1 = (x_2 - x) / (x_2 - x_1);
        let x_slope2 = (x - x_1) / (x_2 - x_1);
        let y_slope1 = (y_2 - y) / (y_2 - y_1);
        let y_slope2 = (y - y_1) / (y_2 - y_1);

        let f_x_y1 = f_1_1 * x_slope1 + f_2_1 * x_slope2;
        let f_x_y2 = f_1_2 * x_slope1 + f_2_2 * x_slope2;

        f_x_y1 * y_slope1 + f_x_y2 * y_slope2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    type LinearAxis = axis::Axis<f64, search::Linear, bound::Interp, bound::Interp>;
    type TableLinLin = LookupTable2D<LinearAxis, LinearAxis, f64>;

    fn func(x: f64, y: f64) -> f64 {
        3. * x + y
    }

    //
    // Table Construction
    //
    fn data() -> (Vec<f64>, Vec<f64>, Array2<f64>) {
        let len = 100;
        let x = ndarray::Array1::linspace(0., 5.0, len).to_vec();
        let y = x.clone();
        let mut f = ndarray::Array2::zeros((x.len(), y.len()));

        for i in 0..x.len() {
            for j in 0..y.len() {
                f[[i, j]] = func(x[i], y[j]);
            }
        }

        (x, y, f)
    }

    fn linear_simple_table() -> LookupTable2D<LinearAxis, LinearAxis, f64> {
        let (x, y, f) = data();
        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        LookupTable2D::new(x, search1, y, search2, f).unwrap()
    }

    //
    // Table Construction Tests
    //

    #[test]
    fn construct_table_repeated_entries_1() {
        let (mut x, y, f) = data();

        x[0] = 0.;
        x[1] = 0.;

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();

        let output: Result<TableLinLin, _> = LookupTable2D::new(x, search1, y, search2, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_entries_2() {
        let (x, mut y, f) = data();

        y[0] = 0.;
        y[1] = 0.;

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();

        let output: Result<TableLinLin, _> = LookupTable2D::new(x, search1, y, search2, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_mismatch_length_1() {
        let (mut x, y, f) = data();

        x.push(100.);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();

        let output: Result<TableLinLin, _> = LookupTable2D::new(x, search1, y, search2, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_mismatch_length_2() {
        let (x, mut y, f) = data();

        y.push(100.0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();

        let output: Result<TableLinLin, _> = LookupTable2D::new(x, search1, y, search2, f);
        assert!(output.is_err());
    }

    #[test]
    /// prove reversing the x vector yields the same lookup results
    fn construct_table_reversed_ax1() {
        let (mut x, y, f) = data();

        x.reverse();

        let y_0 = y[y.len() / 3];
        let x_0 = x[2 * x.len() / 3];
        let f_actual = func(x_0, y_0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();

        let table: TableLinLin = LookupTable2D::new(x, search1, y, search2, f).unwrap();

        let table_reversed_output = table.lookup(x_0, y_0);

        float_eq::assert_float_ne!(table_reversed_output, f_actual, abs <= TOL);
    }

    #[test]
    /// prove reversing the y vector yields the same lookup results
    fn construct_table_reversed_ax2() {
        let (x, mut y, f) = data();

        y.reverse();

        let y_0 = y[y.len() / 3];
        let x_0 = x[2 * x.len() / 3];
        let f_actual = func(x_0, y_0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();

        let table: TableLinLin = LookupTable2D::new(x, search1, y, search2, f).unwrap();

        let table_reversed_output = table.lookup(x_0, y_0);

        float_eq::assert_float_ne!(table_reversed_output, f_actual, abs <= TOL);
    }

    //
    // Linear Tests
    //

    #[test]
    fn linear_1() {
        let table = linear_simple_table();
        let x = 0.5;
        let y = 2.5;
        let output = table.lookup(x, y);
        float_eq::assert_float_eq!(output, func(x, y), abs <= TOL);
    }

    #[test]
    fn linear_low_bound() {
        let table = linear_simple_table();
        let x = 0.;
        let y = 0.;
        let output = table.lookup(x, y);
        float_eq::assert_float_eq!(output, func(x, y), abs <= TOL);
    }

    #[test]
    fn linear_high_bound() {
        let table = linear_simple_table();
        let x = 5.0;
        let y = 5.0;
        let output = table.lookup(x, y);
        float_eq::assert_float_eq!(output, func(x, y), abs <= TOL);
    }
}
