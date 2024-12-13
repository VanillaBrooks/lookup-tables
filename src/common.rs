use crate::Error;

pub(crate) enum IndependentVariableOrdering {
    MonotonicallyIncreasing,
    MonotonicallyDecreasing,
}

/// Ensure all the expected properties of an independent variable axis are upheld
///
/// * Strictly onotonically increasing or decreasing
/// * No duplicate entries
pub(crate) fn check_independent_variable<Indep>(
    indep: &[Indep],
) -> Result<IndependentVariableOrdering, Error>
where
    Indep: std::cmp::PartialOrd,
{
    check_repeat_entries(indep)?;

    // check if mono increasing
    if indep.is_sorted() {
        return Ok(IndependentVariableOrdering::MonotonicallyIncreasing);
    } else {
        // check if monotonically decreasing
        if indep.is_sorted_by(|l, r| r < l) {
            return Ok(IndependentVariableOrdering::MonotonicallyDecreasing);
        }
        // its neither increasing or decreasing, there is an error
        else {
            return Err(Error::NonMonotonicSorting);
            //
        }
    }
}

fn check_repeat_entries<Indep>(indep: &[Indep]) -> Result<(), Error>
where
    Indep: PartialEq<Indep>,
{
    for value in indep {
        if indep.iter().filter(|x| *x == value).count() != 1 {
            return Err(Error::DuplicateEntry);
        }
    }
    Ok(())
}

pub(crate) fn check_lengths(indep_length: usize, dep_axis_length: usize) -> Result<(), Error> {
    if indep_length != dep_axis_length {
        return Err(Error::IndependentDependentLength);
    }

    if indep_length < 2 || dep_axis_length < 2 {
        return Err(Error::IndependentVariableTooShort);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_same() {
        let output = check_lengths(1, 1);
        assert!(output.is_ok());
    }

    #[test]
    fn length_different() {
        let output = check_lengths(1, 2);
        assert!(output.is_err());
    }

    #[test]
    fn check_repeat_entries_unique() {
        let entries = [1, 2, 3, 4, 5];
        let output = check_repeat_entries(&entries);
        assert!(output.is_ok());
    }

    #[test]
    fn check_repeat_entries_nonunique() {
        let entries = [1, 2, 4, 4, 5];
        let output = check_repeat_entries(&entries);
        assert!(output.is_err());
    }

    #[test]
    fn monotonically_increasing() {
        let entries = [1, 2, 3, 4, 5];
        let output = check_independent_variable(&entries);
        assert!(output.is_ok());
    }

    #[test]
    fn monotonically_decreasing() {
        let entries = [5, 4, 3, 2, 1];
        let output = check_repeat_entries(&entries);
        assert!(output.is_ok());
    }

    #[test]
    fn monotonically_increasing_repeated() {
        let entries = [1, 2, 3, 4, 4, 5];
        let output = check_independent_variable(&entries);
        assert!(output.is_err());
    }

    #[test]
    fn monotonically_decreasing_repeated() {
        let entries = [5, 4, 3, 2, 2, 1];
        let output = check_repeat_entries(&entries);
        assert!(output.is_err());
    }
}
