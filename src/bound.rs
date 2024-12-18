/// Interpolate out of bounds using the first / last two grid points.
///
/// # Example
///
/// ```
/// use lookup_tables::{Axis, Binary, Interp, LookupTable1D};
///
/// // independent variable is `f64`. Searching along the axis uses a binary search method.
/// // interpolation is performed at the far edges of the independent variable.
/// type InterpAxis = Axis<f64, Binary, Interp, Interp>;
///
/// let x = vec![0., 5.0, 10.];
/// // y = 2.0 * x
/// let y = x.iter().map(|value| 2.0 * value).collect();
///
/// let table = LookupTable1D::<InterpAxis, f64>::new(x, Binary::new(), y).unwrap();
///
/// // inbounds is what you expect
/// assert!(table.lookup(5.) == 10.);
///
/// // lower bound is interpolated on. using `f(0.0) = 0` and `f(5.0) = 10.0`
/// assert!(table.lookup(-10.) == -20.); // lower bound is interpolated on. using `f(5.0) = 10.` and `f(10.) = 20.0`
/// assert!(table.lookup(20.) == 40.);
/// ```
pub struct Interp;

/// Clamp interpolation results to the value of the independent variable at the bounds.
///
/// # Example
///
/// ```
/// use lookup_tables::{Axis, Linear, Clamp, LookupTable1D};
///
/// // independent variable is `f64`. Searching along the axis uses a linear method.
/// // clamping is performed at both ends of the axis.
/// type ClampedAxis = Axis<f64, Linear, Clamp, Clamp>;
///
/// let x = vec![0., 5.0, 10.];
/// // y = 2.0 * x
/// let y = x.iter().map(|value| 2.0 * value).collect();
///
/// let table = LookupTable1D::<ClampedAxis, f64>::new(x, Linear::new(), y).unwrap();
///
/// // inbounds is what you expect
/// assert!(table.lookup(5.) == 10.);
///
/// // lower bound is clamped, saturates to f(x) = 2 * 0 = 0
/// assert!(table.lookup(-10.) == 0.);
/// // upper bound is clamped, saturates to f(x) = 2 * 10 = 20
/// assert!(table.lookup(20.) == 20.);
/// ```
pub struct Clamp;

/// Defines how to treat a lookup of an independent variable its upper and lower bounds.
pub trait Bound<Indep> {
    /// Behavior at the upper bound.
    fn upper_bound(indep: Indep, upper_bound: Indep) -> Indep;

    /// Behavior at the lower bound.
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

#[cfg(test)]
mod tests {
    use super::*;

    //
    // Clamp Tests
    //

    #[test]
    fn clamp_inbounds() {
        let x = 2.5;
        let lower = 0.0;
        let higher = 5.0;
        let output_high = Clamp::upper_bound(x, higher);
        let output_low = Clamp::lower_bound(x, lower);

        assert!(output_high == x);
        assert!(output_low == x);
    }

    #[test]
    fn clamp_high() {
        let x = 10.0;
        let lower = 0.0;
        let higher = 5.0;
        let output_high = Clamp::upper_bound(x, higher);
        let output_low = Clamp::lower_bound(x, lower);

        assert!(output_high == higher);
        assert!(output_low == x);
    }

    #[test]
    fn clamp_low() {
        let x = -2.0;
        let lower = 0.0;
        let higher = 5.0;
        let output_high = Clamp::upper_bound(x, higher);
        let output_low = Clamp::lower_bound(x, lower);

        assert!(output_high == x);
        assert!(output_low == lower);
    }

    //
    // Interp Tests
    //

    #[test]
    fn interp_inbounds() {
        let x = 2.5;
        let lower = 0.0;
        let higher = 5.0;
        let output_high = Interp::upper_bound(x, higher);
        let output_low = Interp::lower_bound(x, lower);

        assert!(output_high == x);
        assert!(output_low == x);
    }

    #[test]
    fn interp_high() {
        let x = 10.0;
        let lower = 0.0;
        let higher = 5.0;
        let output_high = Interp::upper_bound(x, higher);
        let output_low = Interp::lower_bound(x, lower);

        assert!(output_high == x);
        assert!(output_low == x);
    }

    #[test]
    fn interp_low() {
        let x = -2.0;
        let lower = 0.0;
        let higher = 5.0;
        let output_high = Interp::upper_bound(x, higher);
        let output_low = Interp::lower_bound(x, lower);

        assert!(output_high == x);
        assert!(output_low == x);
    }
}
