use crate::Error;

pub(crate) enum IndependentVariableOrdering {
    MonotonicallyIncreasing,
    MonotonicallyDecreasing,
}

pub(crate) fn check_independent_variable<Indep>(
    indep: &[Indep],
) -> Result<IndependentVariableOrdering, Error>
where
    Indep: std::cmp::PartialOrd,
{
    // if the independent variable is not monotonically increasing...
    if indep.is_sorted() {
        return Ok(IndependentVariableOrdering::MonotonicallyIncreasing);
    } else {
        // if it is monotonically decreasing, just reverse the data so our lookups can treat
        // it as monotonically increasing
        if indep.is_sorted_by(|l, r| r < l) {
            return Ok(IndependentVariableOrdering::MonotonicallyDecreasing);
        } else {
            return Err(Error::NonMonotonicSorting);
            //
        }
    }
}

pub(crate) fn check_lengths(indep_length: usize, dep_axis_length: usize) -> Result<(), Error> {
    if indep_length != dep_axis_length {
        Err(Error::IndependentDependentLength)
    } else {
        Ok(())
    }
}
