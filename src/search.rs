// todo: constructors for these instead of default
// todo: fixed delta search method

/// Linear search to bind the bounding indices. Typically faster for small
/// (<20) values in the table.
#[derive(Default)]
pub struct Linear;

#[derive(Default)]
/// Binary search to find bounding indices. Useful for large datasets (>20)
pub struct Binary;

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

/// Determine search method dynamically at runtime
pub enum Runtime {
    Linear(Linear),
    Binary(Binary),
    CachedLinearCell(CachedLinearCell),
}

pub trait Search<Indep>
where
    Indep: PartialOrd<Indep>,
{
    /// Search through a list of values, return the upper and lower indices that bound a given
    /// value
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize);
}

fn inbounds_pair_from_lower(low_idx: usize, indep_length: usize) -> (usize, usize) {
    // cap the low index to be two minus the length, as one minus the length would
    // put the high index out of bounds
    let low_idx = std::cmp::min(low_idx, indep_length - 2);
    let high_idx = low_idx + 1;

    (low_idx, high_idx)
}

fn inbounds_pair_from_higher(high_idx: usize) -> (usize, usize) {
    // cap the high index to be 1 to ensure the lower index is inbounds at zero
    let high_idx = std::cmp::max(1, high_idx);
    let low_idx = high_idx - 1;

    (low_idx, high_idx)
}

impl<Indep> Search<Indep> for Linear
where
    Indep: std::cmp::PartialOrd,
{
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        let length = indep_values.len();

        if let Some(low_idx) = indep_values.iter().position(|v| v > value) {
            // grab the index pair associated with this, paying close attention to not go out of
            // bounds
            inbounds_pair_from_lower(low_idx, length)
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
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        let length = indep_values.len();
        let f = |v: &Indep| v.partial_cmp(value).unwrap();

        match indep_values.binary_search_by(f) {
            Ok(matching_index) => inbounds_pair_from_lower(matching_index, length),
            Err(low_idx) => inbounds_pair_from_lower(low_idx, length),
        }
    }
}

impl<Indep> Search<Indep> for CachedLinearCell
where
    Indep: PartialOrd<Indep>,
{
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        let mut borrow_idx: std::cell::RefMut<'_, usize> = self
            .last_lower_idx
            .try_borrow_mut()
            .expect("Cached RefCell was already borrowed. This should never happen");
        let last_lower: usize = *borrow_idx;

        let length = indep_values.len();

        if &indep_values[last_lower] >= value {
            // we need to search the lower portion of the dataset since our value is smaller than
            // the last index

            for idx in last_lower..0 {
                dbg!("here");
                let idx_value = &indep_values[idx];
                if idx_value < value {
                    // we are now at an index that is above the value, we return out
                    let index_pair = inbounds_pair_from_higher(idx);
                    *borrow_idx = index_pair.0;
                    return index_pair;
                }
            }

            // TODO: not sure if this is a band aid on larger problem, but we can reach this code
            // if the last lower index was 0. The for loop above will not run, and we need to
            // return something
            let index_pair = (0, 1);
            *borrow_idx = index_pair.0;
            return index_pair;
        } else {
            for idx in last_lower..length {
                let idx_value = &indep_values[idx];
                if idx_value < value {
                    // we are now at an index that is above the value, we return out
                    let index_pair = inbounds_pair_from_higher(idx);
                    *borrow_idx = index_pair.0;
                    return index_pair;
                }
            }

            unreachable!()
        }
    }
}

impl<Indep> Search<Indep> for Runtime 
where
    Indep: PartialOrd<Indep>,
{
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        match &self {
            Runtime::Linear(l) => l.search(value, indep_values),
            Runtime::Binary(b) => b.search(value, indep_values),
            Runtime::CachedLinearCell(c) => c.search(value, indep_values),
        }
    }
}
