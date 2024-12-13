// todo: constructors for these instead of default
// todo: search should specify the behavior at the bounds

#[derive(Default)]
pub struct Linear;

#[derive(Default)]
pub struct Binary;

#[derive(Default)]
pub struct CachedLinearCell;

pub trait Search<Indep> 
where Indep: PartialOrd<Indep>,
{
    /// Search through a list of values, return the upper and lower indices that bound a given
    /// value
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize);
}

impl<Indep> Search<Indep> for Linear 
where Indep: std::cmp::PartialOrd,
{
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        let length = indep_values.len();

        if let Some(low_idx) = indep_values.iter().position(|v| v > value) {
            // we can at most have length - 2 since the high side would be length - 1
            // which is the end of the vector
            let low_idx = std::cmp::min(low_idx, length -2);
            let high_idx = low_idx + 1;

            (low_idx, high_idx)
        } else {
            // we hit the max value in the dataset and are were not bigger. set the high
            // index equal to the last value 
            let high_idx = length-1;
            let low_idx = high_idx -1;

            (low_idx, high_idx)
        }

    }
}

impl<Indep> Search<Indep> for Binary 
where Indep: PartialOrd<Indep>,
{
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        //
        todo!()
    }
}

impl<Indep> Search<Indep> for CachedLinearCell 
where Indep: PartialOrd<Indep>,
{
    fn search(&self, value: &Indep, indep_values: &[Indep]) -> (usize, usize) {
        //
        todo!()
    }
}
