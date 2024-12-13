# lookup-tables

High performance & compile-time customizable lookup tables


## Tables

* [`LookupTable1D`]

## Out-of-bounds behavior per-axis

* [`Clamp`] - Clamp at the bounds and do not extrapolate outside the table
* [`Interp`] - Interpolate freely outside bounds

## Searching Methods

* [`Linear`] - Lineary search to find bounding indicies. Typically faster for small (`< 20`) values in a table
* [`Binary`] - Binary search for bounding indices. Useful for large datasets
* [`CachedLinearCell`] - Linear searching with a cached last-used index. Effective for large datasets with slowly changing lookup values
* [`RuntimeSearch`] - Use any search method, configured at runtime

## Axis Customization

Interpolation search and bounding is configured on a per [`Axis`] basis. An axis consists of

* `Indep` - type of the independent variable. Typically [`f32`] or [`f64`]
* `Search` - search method implementing the [`Search`] trait
* `LowerBound` - bounding behavior at the lower bound of the axis implementing the [`Bound`] trait
* `UpperBound` - bounding behavior at the higher bound of the axis implementing the [`Bound`] trait


## Examples

### Lookup Table 1D With Clamping on Lower Bound

```rust
use lookup_tables::{Axis, Linear, Clamp, Interp, LookupTable1D};
use std::f64::consts::PI;

let radius = 1.0;
let height_data  = vec![0., 1., 2., 3., 4., 5.];
// experimentally measured volume an irregular object at the above heights
let volume_data  = vec![0., 3., 5., 10., 12., 13.];

// height data will be f64 and searched with a linear method. The lower bound will be clamped as we cannot have
// a height / volume less than 0, but the upper bound will interpolate unbounded
type MyAxis = Axis<f64, Linear, Clamp, Interp>;

// lookup table will search through the first independent variable (height) with parameters from `MyAxis`. Dependent
// variable (volume) returned will be `f64`
type MyTable = LookupTable1D<MyAxis, f64>;

let table = MyTable::new(height_data, Linear::default(), volume_data).unwrap();

let interpolated_volume = table.lookup(&2.5);
assert!(interpolated_volume == 7.5);

// negative height measurement clamps the results to the lowest volume
let interpolated_volume = table.lookup(&-1.0);
assert!(interpolated_volume == 0.0);

// out of bounds height interpolates volume linearly
let interpolated_volume = table.lookup(&10.);
assert!(interpolated_volume == 18.0);
```

### Lookup Table 1D With Vector-Valued Dependent Variable

Instead of a table to convert a height measurement to a volume reading, what if we wanted to compute multiple
properties of our system simultaneously using the same height data? We could construct multiple tables or 
we could change the dependent variable to an array type meeting the bounds of [`LookupTable1D::lookup`]

```rust
use lookup_tables::{Axis, Linear, Clamp, Interp, LookupTable1D};
use nalgebra::Vector2;
use std::f64::consts::PI;

let radius = 1.0;
let height_data  = vec![0., 1., 2., 3., 4., 5.];
// experimentally measured property data of an irregular object at the above heights.
// the first entry in each index is the same as our volume data above
let property_data = vec![
    Vector2::new(0., 0.), 
    Vector2::new(3., 1.),
    Vector2::new(5., 2.), 
    Vector2::new(10.,3.),
    Vector2::new(12.,4.),
    Vector2::new(13.,5.)
];

// Same axis type as above. Contains `f64` data, searched with a linear method, clamped at
// the lower bound, interpolated at the higher bound.
type MyAxis = Axis<f64, Linear, Clamp, Interp>;

// lookup table will search through the first independent variable (height) with parameters from `MyAxis`. 
// Dependent variable returned will be `Vector2`
type MyTable = LookupTable1D<MyAxis, Vector2<f64>>;

let table = MyTable::new(height_data, Linear::default(), property_data).unwrap();

let interpolated_volume = table.lookup(&2.5);
assert!(interpolated_volume == Vector2::new(7.5, 2.5));
```
