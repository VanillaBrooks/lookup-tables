mod axis;
mod common;
mod search;
mod table1d;

#[derive(Debug)]
enum Error {
    NonMonotonicSorting,
    DuplicateEntry,
    IndependentDependentLength,
}
