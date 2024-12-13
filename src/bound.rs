pub struct Interp;

pub struct Clamp;

pub trait Bound<Indep> {
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
