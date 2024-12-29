use crate::axis;
use crate::bound;
use crate::common;
use crate::search;
use crate::Error;
use std::ops::{Add, Div, Mul, Sub};

use ndarray::Array3;
use num_traits::identities::One;

/// Three dimensional lookup table - approximate `f(x, y, z)` given `x`, `y`, and `z`
///
/// See [crate level](crate) documentation for more examples and usage
pub struct LookupTable3D<Axis1, Axis2, Axis3, Dep>
where
    Axis1: axis::AxisImpl,
    Axis2: axis::AxisImpl,
    Axis3: axis::AxisImpl,
{
    indep1: Vec<<Axis1 as axis::AxisImpl>::Indep>,
    search1: <Axis1 as axis::AxisImpl>::Search,
    indep2: Vec<<Axis2 as axis::AxisImpl>::Indep>,
    search2: <Axis2 as axis::AxisImpl>::Search,
    indep3: Vec<<Axis3 as axis::AxisImpl>::Indep>,
    search3: <Axis3 as axis::AxisImpl>::Search,
    dep: Array3<Dep>,
}

impl<
        Indep1,
        Search1,
        LowerBound1,
        UpperBound1,
        Indep2,
        Search2,
        LowerBound2,
        UpperBound2,
        Indep3,
        Search3,
        LowerBound3,
        UpperBound3,
        Dep,
    >
    LookupTable3D<
        axis::Axis<Indep1, Search1, LowerBound1, UpperBound1>,
        axis::Axis<Indep2, Search2, LowerBound2, UpperBound2>,
        axis::Axis<Indep3, Search3, LowerBound3, UpperBound3>,
        Dep,
    >
where
    Indep1: std::cmp::PartialOrd,
    Indep2: std::cmp::PartialOrd,
    Indep3: std::cmp::PartialOrd,
{
    /// Construct a new lookup table
    ///
    /// # Args
    ///
    /// ## `indep1`
    ///
    /// List of independent variables (`x` in `f(x, y, z)`). `Indep1` is generally `f64` or `f32`.
    ///
    /// ## `search1`
    ///
    /// Search method for `indep1`. Implements the [Search](crate::search::Search) trait.
    ///
    ///
    /// ## `indep2`
    ///
    /// List of independent variables (`y` in `f(x, y, z)`). `Indep2` is generally `f64` or `f32`.
    ///
    /// ## `search2`
    ///
    /// Search method for `indep2`. Implements the [Search](crate::search::Search) trait.
    ///
    /// ## `indep3`
    ///
    /// List of independent variables (`z` in `f(x, y, z)`). `Indep3` is generally `f64` or `f32`.
    ///
    /// ## `search3`
    ///
    /// Search method for `indep3`. Implements the [Search](crate::search::Search) trait.
    ///
    /// ## `dep`
    ///
    /// List of dependent variables (`f(x, y, z)`). `Dep` is generally `f64`, `f32`, some vector valued `nalgebra::base::Vector`, or
    /// [ndarray::Array1]
    ///
    /// # Example
    ///
    /// ```
    /// use lookup_tables::{Linear, Binary, Axis, Interp, Clamp, LookupTable3D};
    ///
    /// // independent variable axis of `f64`s. Searching the axis will be done with a brute force
    /// // linear search (good for < 20 values). No clamping at the bounds of the table
    /// type LinearInterpAxis = Axis<f64, Linear, Interp, Interp>;
    ///
    /// // independent variable axis of `f64`s. Searching the axis will be done with binary search.
    /// // Clamping at the upper bound, interpolation at the lower bound.
    /// type BinaryClampLowerAxis = Axis<f64, Binary, Clamp, Interp>;
    ///
    /// // independent variable axis of `f64`s. Searching the axis will be done with a brute force
    /// type LinearClampAxis = Axis<f64, Linear, Clamp, Clamp>;
    ///
    /// // our table uses three different lookup methods on each of the three different axes. the
    /// // dependent variable is a `f64`, based on what the `f` function returns below!
    /// type Table3D = LookupTable3D::<LinearInterpAxis, BinaryClampLowerAxis, LinearClampAxis, f64>;
    ///
    /// let x = vec![1., 2., 3.];
    /// let y = vec![10., 20., 30.];
    /// let z = vec![2., 4., 8., 12.];
    /// // f(x,y) = x + y
    /// let f = |x, y, z| x + y + z;
    /// let mut f_matrix = ndarray::Array3::zeros((x.len(), y.len(), z.len()));
    ///
    /// //populate the f matrix with function evaluations
    /// for i in 0..x.len() {
    ///     for j in 0..y.len() {
    ///         for k in 0..y.len() {
    ///             f_matrix[[i,j,k]] = f(x[i], y[j], z[k]);
    ///         }
    ///     }
    /// }
    ///
    /// // construct the 3d lookup table
    /// let table = Table3D::new(x, Linear::new(), y, Binary::new(), z, Linear::new(), f_matrix).unwrap();
    /// ```
    pub fn new(
        mut indep1: Vec<Indep1>,
        search1: Search1,
        mut indep2: Vec<Indep2>,
        search2: Search2,
        mut indep3: Vec<Indep3>,
        search3: Search3,
        mut dep: Array3<Dep>,
    ) -> Result<Self, Error> {
        match common::check_independent_variable(indep1.as_slice())? {
            common::IndependentVariableOrdering::MonotonicallyIncreasing => {}
            common::IndependentVariableOrdering::MonotonicallyDecreasing => {
                indep1.reverse();
                dep.invert_axis(ndarray::Axis(0));
            }
        }

        match common::check_independent_variable(indep2.as_slice())? {
            common::IndependentVariableOrdering::MonotonicallyIncreasing => {}
            common::IndependentVariableOrdering::MonotonicallyDecreasing => {
                indep2.reverse();
                dep.invert_axis(ndarray::Axis(1));
            }
        }

        match common::check_independent_variable(indep3.as_slice())? {
            common::IndependentVariableOrdering::MonotonicallyIncreasing => {}
            common::IndependentVariableOrdering::MonotonicallyDecreasing => {
                indep3.reverse();
                dep.invert_axis(ndarray::Axis(2));
            }
        }

        common::check_lengths(indep1.len(), dep.len_of(ndarray::Axis(0)))?;
        common::check_lengths(indep2.len(), dep.len_of(ndarray::Axis(1)))?;
        common::check_lengths(indep3.len(), dep.len_of(ndarray::Axis(2)))?;

        Ok(Self {
            indep1,
            search1,
            indep2,
            search2,
            indep3,
            search3,
            dep,
        })
    }
}

impl<
        Indep1,
        Search1,
        LowerBound1,
        UpperBound1,
        Indep2,
        Search2,
        LowerBound2,
        UpperBound2,
        Indep3,
        Search3,
        LowerBound3,
        UpperBound3,
        Dep,
    >
    LookupTable3D<
        axis::Axis<Indep1, Search1, LowerBound1, UpperBound1>,
        axis::Axis<Indep2, Search2, LowerBound2, UpperBound2>,
        axis::Axis<Indep3, Search3, LowerBound3, UpperBound3>,
        Dep,
    >
where
    Search1: search::Search<Indep1>,
    Search2: search::Search<Indep2>,
    Search3: search::Search<Indep3>,
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
        + One
        //
        + std::fmt::Debug,
    Indep2: Copy
        + Sub<Indep2, Output = Indep2>
        + std::cmp::PartialOrd
        + Div<Indep2, Output = Indep2>
        + One
        //
        + std::fmt::Debug,
    Indep3: Copy
        + Sub<Indep2, Output = Indep2>
        + std::cmp::PartialOrd
        + Div<Indep2, Output = Indep2>
        + One
        //
        + std::fmt::Debug,
    LowerBound1: bound::Bound<Indep1>,
    UpperBound1: bound::Bound<Indep1>,
    LowerBound2: bound::Bound<Indep2>,
    UpperBound2: bound::Bound<Indep2>,
    LowerBound3: bound::Bound<Indep3>,
    UpperBound3: bound::Bound<Indep3>,
{
    pub fn lookup(&self, x: Indep1, y: Indep2, z: Indep3) -> Dep {
        let (idx_x_1, idx_x_2) = self.search1.search(x, self.indep1.as_slice());
        let (idx_y_1, idx_y_2) = self.search2.search(y, self.indep2.as_slice());
        let (idx_z_1, idx_z_2) = self.search3.search(z, self.indep3.as_slice());

        let x_1: Indep1 = self.indep1[idx_x_1];
        let x_2: Indep1 = self.indep1[idx_x_2];

        let y_1: Indep2 = self.indep2[idx_y_1];
        let y_2: Indep2 = self.indep2[idx_y_2];

        let z_1: Indep2 = self.indep2[idx_z_1];
        let z_2: Indep2 = self.indep2[idx_z_2];

        // function evaluations at the bounding indices
        let f_1_1_1: Dep = self.dep[[idx_x_1, idx_y_1, idx_z_1]];
        let f_2_1_1: Dep = self.dep[[idx_x_2, idx_y_1, idx_z_1]];
        let f_1_1_2: Dep = self.dep[[idx_x_1, idx_y_1, idx_z_2]];
        let f_2_1_2: Dep = self.dep[[idx_x_2, idx_y_1, idx_z_2]];
        let f_1_2_1: Dep = self.dep[[idx_x_1, idx_y_2, idx_z_1]];
        let f_2_2_1: Dep = self.dep[[idx_x_2, idx_y_2, idx_z_1]];
        let f_1_2_2: Dep = self.dep[[idx_x_1, idx_y_2, idx_z_2]];
        let f_2_2_2: Dep = self.dep[[idx_x_2, idx_y_2, idx_z_2]];

        // bound x acording to the axis we are interpolating on
        // unwrap is safe here as we have checked its at least length 2
        let x = LowerBound1::lower_bound(x, *self.indep1.first().unwrap());
        let x = UpperBound1::upper_bound(x, *self.indep1.last().unwrap());

        // bound y acording to the axis we are interpolating on
        // unwrap is safe here as we have checked its at least length 2
        let y = LowerBound2::lower_bound(y, *self.indep2.first().unwrap());
        let y = UpperBound2::upper_bound(y, *self.indep2.last().unwrap());

        // bound z acording to the axis we are interpolating on
        // unwrap is safe here as we have checked its at least length 2
        let z = LowerBound3::lower_bound(z, *self.indep3.first().unwrap());
        let z = UpperBound3::upper_bound(z, *self.indep3.last().unwrap());

        let x_d = (x - x_1) / (x_2 - x_1);
        let y_d = (y - y_1) / (y_2 - y_1);
        let z_d = (z - z_1) / (z_2 - z_1);

        let f_1_1 = f_1_1_1 * (Indep1::one() - x_d) + f_2_1_1 * x_d;
        let f_1_2 = f_1_1_2 * (Indep1::one() - x_d) + f_2_1_2 * x_d;
        let f_2_1 = f_1_2_1 * (Indep1::one() - x_d) + f_2_2_1 * x_d;
        let f_2_2 = f_1_2_2 * (Indep1::one() - x_d) + f_2_2_2 * x_d;

        let f_1 = f_1_1 * (Indep2::one() - y_d) + f_2_1 * y_d;
        let f_2 = f_1_2 * (Indep2::one() - y_d) + f_2_2 * y_d;

        f_1 * (Indep3::one() - z_d) + f_2 * z_d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-10;

    type LinearAxis = axis::Axis<f64, search::Linear, bound::Interp, bound::Interp>;
    type TableLinLinLin = LookupTable3D<LinearAxis, LinearAxis, LinearAxis, f64>;

    fn func(x: f64, y: f64, z: f64) -> f64 {
        3. * x + y + x + z
    }

    //
    // Table Construction
    //
    fn data() -> (Vec<f64>, Vec<f64>, Vec<f64>, Array3<f64>) {
        let len = 100;
        let x = ndarray::Array1::linspace(0., 5.0, len).to_vec();
        let y = x.clone();
        let z = x.clone();
        let mut f = ndarray::Array3::zeros((x.len(), y.len(), z.len()));

        for i in 0..x.len() {
            for j in 0..y.len() {
                for k in 0..y.len() {
                    f[[i, j, k]] = func(x[i], y[j], z[k]);
                }
            }
        }

        (x, y, z, f)
    }

    fn linear_simple_table() -> LookupTable3D<LinearAxis, LinearAxis, LinearAxis, f64> {
        let (x, y, z, f) = data();
        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();
        LookupTable3D::new(x, search1, y, search2, z, search3, f).unwrap()
    }

    //
    // Table Construction Tests
    //

    #[test]
    fn construct_table_repeated_entries_1() {
        let (mut x, y, z, f) = data();

        x[0] = 0.;
        x[1] = 0.;

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let output: Result<TableLinLinLin, _> =
            LookupTable3D::new(x, search1, y, search2, z, search3, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_entries_2() {
        let (x, mut y, z, f) = data();

        y[0] = 0.;
        y[1] = 0.;

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let output: Result<TableLinLinLin, _> =
            LookupTable3D::new(x, search1, y, search2, z, search3, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_entries_3() {
        let (x, y, mut z, f) = data();

        z[0] = 0.;
        z[1] = 0.;

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let output: Result<TableLinLinLin, _> =
            LookupTable3D::new(x, search1, y, search2, z, search3, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_mismatch_length_1() {
        let (mut x, y, z, f) = data();

        x.push(100.);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let output: Result<TableLinLinLin, _> =
            LookupTable3D::new(x, search1, y, search2, z, search3, f);
        assert!(output.is_err());
    }

    #[test]
    fn construct_table_repeated_mismatch_length_2() {
        let (x, mut y, z, f) = data();

        y.push(100.0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let output: Result<TableLinLinLin, _> =
            LookupTable3D::new(x, search1, y, search2, z, search3, f);
        assert!(output.is_err());
    }

    #[test]
    /// prove reversing the x vector yields the same lookup results
    fn construct_table_reversed_ax1() {
        let (mut x, y, z, f) = data();

        x.reverse();

        let y_0 = y[y.len() / 3];
        let x_0 = x[2 * x.len() / 3];
        let z_0 = z[z.len() / 4];
        let f_actual = func(x_0, y_0, z_0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let table: TableLinLinLin =
            LookupTable3D::new(x, search1, y, search2, z, search3, f).unwrap();

        let table_reversed_output = table.lookup(x_0, y_0, z_0);

        float_eq::assert_float_ne!(table_reversed_output, f_actual, abs <= TOL);
    }

    #[test]
    /// prove reversing the y vector yields the same lookup results
    fn construct_table_reversed_ax2() {
        let (x, mut y, z, f) = data();

        y.reverse();

        let y_0 = y[y.len() / 3];
        let x_0 = x[2 * x.len() / 3];
        let z_0 = z[z.len() / 4];
        let f_actual = func(x_0, y_0, z_0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let table: TableLinLinLin =
            LookupTable3D::new(x, search1, y, search2, z, search3, f).unwrap();

        let table_reversed_output = table.lookup(x_0, y_0, z_0);

        float_eq::assert_float_ne!(table_reversed_output, f_actual, abs <= TOL);
    }

    #[test]
    /// prove reversing the z vector yields the same lookup results
    fn construct_table_reversed_ax3() {
        let (x, y, mut z, f) = data();

        z.reverse();

        let y_0 = y[y.len() / 3];
        let x_0 = x[2 * x.len() / 3];
        let z_0 = z[z.len() / 4];
        let f_actual = func(x_0, y_0, z_0);

        let search1 = search::Linear::default();
        let search2 = search::Linear::default();
        let search3 = search::Linear::default();

        let table: TableLinLinLin =
            LookupTable3D::new(x, search1, y, search2, z, search3, f).unwrap();

        let table_reversed_output = table.lookup(x_0, y_0, z_0);

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
        let z = 4.2;
        let output = table.lookup(x, y, z);
        float_eq::assert_float_eq!(output, func(x, y, z), abs <= TOL);
    }

    #[test]
    fn linear_low_bound() {
        let table = linear_simple_table();
        let x = 0.;
        let y = 0.;
        let z = 0.;
        let output = table.lookup(x, y, z);
        float_eq::assert_float_eq!(output, func(x, y, z), abs <= TOL);
    }

    #[test]
    fn linear_high_bound() {
        let table = linear_simple_table();
        let x = 5.0;
        let y = 5.0;
        let z = 5.0;
        let output = table.lookup(x, y, z);
        float_eq::assert_float_eq!(output, func(x, y, z), abs <= TOL);
    }
}
