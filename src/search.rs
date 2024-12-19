// todo: constructors for these instead of default
// todo: fixed delta search method

/// Linear search to find the bounding indices. Typically faster for small
/// (<20) values in the table.
#[derive(Default)]
pub struct Linear;

impl Linear {
    /// Construct a new linear search method
    pub fn new() -> Self {
        Linear
    }
}

#[derive(Default)]
/// Binary search to find bounding indices. Useful for large datasets (>20).
pub struct Binary;

impl Binary {
    /// Construct a new binary search method
    pub fn new() -> Self {
        Binary
    }
}

/// Store the last set of bounding indices and learly search from the last known match. Effective
/// for tables with slowly changing values.
#[derive(Default)]
pub struct CachedLinearCell {
    last_lower_idx: std::cell::RefCell<usize>,
}

impl CachedLinearCell {
    pub fn new(last_index: usize) -> Self {
        Self {
            last_lower_idx: last_index.into(),
        }
    }
}

/// Determine search method dynamically at runtime.
pub enum RuntimeSearch {
    Linear(Linear),
    Binary(Binary),
    CachedLinearCell(CachedLinearCell),
}

/// Find the two bounding indices in a vector for interpolation.
pub trait Search<Indep>
where
    Indep: PartialOrd<Indep>,
{
    /// Search through a list of values, return the upper and lower indices that bound a given
    /// value
    fn search(&self, value: Indep, indep_values: &[Indep]) -> (usize, usize);
}

fn inbounds_pair_from_lower(low_idx: usize, indep_length: usize) -> (usize, usize) {
    // cap the low index to be two minus the length, as one minus the length would
    // put the high index out of bounds
    let low_idx = std::cmp::min(low_idx, indep_length - 2);
    let high_idx = low_idx + 1;

    (low_idx, high_idx)
}

fn inbounds_pair_from_higher(high_idx: usize, length: usize) -> (usize, usize) {
    // cap the high index to be 1 to ensure the lower index is inbounds at zero
    let high_idx = std::cmp::max(1, high_idx);
    // ensure the high index is not greater than length -1, which can happen with binary search
    // TODO: const param here to decide if we need to check this, since we dont for the other cases
    let high_idx = std::cmp::min(high_idx, length - 1);

    let low_idx = high_idx - 1;

    (low_idx, high_idx)
}

impl<Indep> Search<Indep> for Linear
where
    Indep: std::cmp::PartialOrd,
{
    fn search(&self, value: Indep, indep_values: &[Indep]) -> (usize, usize) {
        let length = indep_values.len();

        if let Some(high_idx) = indep_values.iter().position(|v| v > &value) {
            // grab the index pair associated with this, paying close attention to not go out of
            // bounds
            inbounds_pair_from_higher(high_idx, length)
        } else {
            // we hit the max value in the dataset and `value` was bigger. set the high
            // index equal to the last value
            let high_idx = length - 1;
            let low_idx = high_idx - 1;

            (low_idx, high_idx)
        }
    }
}

impl<Indep> Search<Indep> for Binary
where
    Indep: PartialOrd<Indep>,
{
    fn search(&self, value: Indep, indep_values: &[Indep]) -> (usize, usize) {
        let length = indep_values.len();
        let f = |v: &Indep| v.partial_cmp(&value).unwrap();

        match indep_values.binary_search_by(f) {
            Ok(matching_index) => inbounds_pair_from_lower(matching_index, length),
            Err(high_idx) => inbounds_pair_from_higher(high_idx, length),
        }
    }
}

impl<Indep> Search<Indep> for CachedLinearCell
where
    Indep: PartialOrd<Indep>,
{
    fn search(&self, value: Indep, indep_values: &[Indep]) -> (usize, usize) {
        let mut borrow_idx: std::cell::RefMut<'_, usize> = self
            .last_lower_idx
            .try_borrow_mut()
            .expect("Cached RefCell was already borrowed. This should never happen");
        let last_lower: usize = *borrow_idx;

        let length = indep_values.len();

        if indep_values[last_lower] >= value {
            // we need to search the lower portion of the dataset since our value is smaller than
            // the last index

            for idx in (0..last_lower).rev() {
                let idx_value = &indep_values[idx];
                if idx_value < &value {
                    // we are now at an index that is above the value, we return out
                    let index_pair = inbounds_pair_from_lower(idx, length);
                    *borrow_idx = index_pair.0;
                    return index_pair;
                }
            }

            let index_pair = (0, 1);
            *borrow_idx = index_pair.0;
            return index_pair;
        } else {
            for idx in last_lower..length {
                let idx_value = &indep_values[idx];
                if idx_value > &value {
                    // we are now at an index that is above the value, we return out
                    let index_pair = inbounds_pair_from_higher(idx, length);
                    *borrow_idx = index_pair.0;
                    return index_pair;
                }
            }

            let index_pair = (length - 2, length - 1);
            *borrow_idx = index_pair.0;
            return index_pair;
        }
    }
}

impl<Indep> Search<Indep> for RuntimeSearch
where
    Indep: PartialOrd<Indep>,
{
    fn search(&self, value: Indep, indep_values: &[Indep]) -> (usize, usize) {
        match &self {
            RuntimeSearch::Linear(l) => l.search(value, indep_values),
            RuntimeSearch::Binary(b) => b.search(value, indep_values),
            RuntimeSearch::CachedLinearCell(c) => c.search(value, indep_values),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Vec<usize> {
        vec![0, 2, 4, 6, 8, 10]
    }

    //
    // Linear Tests
    //

    #[test]
    /// check close to the bottom of the table bounds, but still in
    fn linear_low() {
        let linear = Linear::default();
        let x = data();
        let output = linear.search(1, x.as_slice());
        dbg!(&output);
        assert!(output.0 == 0);
        assert!(output.1 == 1);
    }

    #[test]
    /// check close to the top of the table bounds, but still in
    fn linear_high() {
        let linear = Linear::default();
        let x = data();
        let output = linear.search(9, x.as_slice());
        assert!(output.0 == 4);
        assert!(output.1 == 5);
    }

    //
    // Binary Tests
    //

    #[test]
    /// check close to the bottom of the table bounds, but still in
    fn binary_low() {
        let binary = Binary::default();
        let x = data();
        let output = binary.search(1, x.as_slice());
        dbg!(&output);
        assert!(output.0 == 0);
        assert!(output.1 == 1);
    }

    #[test]
    /// check close to the bottom of the table bounds, but still in
    fn binary_inbounds() {
        let binary = Binary::default();
        let x = data();
        let output = binary.search(5, x.as_slice());
        dbg!(&output);
        assert!(output.0 == 2);
        assert!(output.1 == 3);
    }

    #[test]
    /// check close to the top of the table bounds, but still in
    fn binary_high() {
        let binary = Binary::default();
        let x = data();
        let output = binary.search(9, x.as_slice());
        assert!(output.0 == 4);
        assert!(output.1 == 5);
    }

    //
    // Cached Linear Tests
    //

    #[test]
    /// check close to the bottom of the table bounds, but still in
    fn cached_linear_low() {
        for starting_index in 0..6 {
            dbg!(starting_index);
            let cached_linear = CachedLinearCell::new(starting_index);
            let x = data();
            let output = cached_linear.search(1, x.as_slice());
            dbg!(&output);
            assert!(output.0 == 0);
            assert!(output.1 == 1);
        }
    }

    #[test]
    /// check close to the bottom of the table bounds, but still in
    fn cached_linear_inbounds() {
        for starting_index in 0..6 {
            dbg!(starting_index);
            let cached_linear = CachedLinearCell::new(starting_index);
            let x = data();
            let output = cached_linear.search(5, x.as_slice());
            dbg!(&output);
            assert!(output.0 == 2);
            assert!(output.1 == 3);
        }
    }

    #[test]
    /// check close to the top of the table bounds, but still in
    fn cached_linear_high() {
        for starting_index in 0..6 {
            dbg!(starting_index);
            let cached_linear = CachedLinearCell::new(starting_index);
            let x = data();
            let output = cached_linear.search(9, x.as_slice());
            dbg!(output);
            assert!(output.0 == 4);
            assert!(output.1 == 5);
        }
    }
}
