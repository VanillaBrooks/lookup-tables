use lookup_tables::{Axis, Binary, Bound, CachedLinearCell, Clamp, Interp, Linear, LookupTable1D};
use ndarray_rand::rand::prelude::StdRng;
use ndarray_rand::rand::SeedableRng;

use std::hint::black_box;
use tango_bench::{benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

type AxisLinearClamped = Axis<f64, Linear, Clamp, Clamp>;

type AxisLinearInterp = Axis<f64, Linear, Interp, Interp>;
type AxisBinaryInterp = Axis<f64, Binary, Interp, Interp>;
type AxisCachedLinearInterp = Axis<f64, CachedLinearCell, Interp, Interp>;

fn lookup<Search, LowerBound, UpperBound>(
    table: &LookupTable1D<Axis<f64, Search, LowerBound, UpperBound>, f64>,
    lookup_values: &[f64],
) where
    Search: lookup_tables::Search<f64>,
    LowerBound: Bound<f64>,
    UpperBound: Bound<f64>,
{
    for value in lookup_values {
        table.lookup(black_box(*value));
    }
}

struct BenchPack {
    linear_clamped: &'static LookupTable1D<AxisLinearClamped, f64>,
    linear_interp: &'static LookupTable1D<AxisLinearInterp, f64>,
    binary_interp: &'static LookupTable1D<AxisBinaryInterp, f64>,
    cached_linear_interp: &'static LookupTable1D<AxisCachedLinearInterp, f64>,
    lookup_values: &'static Vec<f64>,
}

impl BenchPack {
    fn new(
        rng: &mut StdRng,
        len: usize,
        num_lookups: usize,
        x_min: f64,
        x_max: f64,
        out_of_bounds_reach: f64,
        sort_lookup_values: bool,
    ) -> Self {
        let mut lookup_values = ndarray::Array1::random_using(
            num_lookups,
            Uniform::new(x_min - out_of_bounds_reach, x_max + out_of_bounds_reach),
            rng,
        )
        .to_vec();

        if sort_lookup_values {
            lookup_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }

        let lookup_values = make_static_ref(lookup_values);

        let mut x = ndarray::Array1::random_using(len, Uniform::new(x_min, x_max), rng).to_vec();
        let y = ndarray::Array1::random_using(len, Uniform::new(-10., 10.), rng).to_vec();
        x.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let linear_clamped = make_static_ref(
            LookupTable1D::<AxisLinearClamped, f64>::new(x.clone(), Linear::default(), y.clone())
                .unwrap(),
        );

        let linear_interp = make_static_ref(
            LookupTable1D::<AxisLinearInterp, f64>::new(x.clone(), Linear::default(), y.clone())
                .unwrap(),
        );

        let binary_interp = make_static_ref(
            LookupTable1D::<AxisBinaryInterp, f64>::new(x.clone(), Binary::default(), y.clone())
                .unwrap(),
        );

        let cached_linear_interp = make_static_ref(
            LookupTable1D::<AxisCachedLinearInterp, f64>::new(
                x.clone(),
                CachedLinearCell::default(),
                y.clone(),
            )
            .unwrap(),
        );

        Self {
            linear_clamped,
            linear_interp,
            binary_interp,
            cached_linear_interp,
            lookup_values,
        }
    }
}

fn make_static_ref<T>(x: T) -> &'static T {
    Box::leak(Box::new(x))
}

fn lookup_random() -> impl IntoBenchmarks {
    let mut rng = StdRng::seed_from_u64(10);

    let mut benchmarks = Vec::new();
    let sort_lookup_values = true;

    for len in [5, 10, 20, 50, 100, 500, 1000] {
        let num_lookups = 10_000_000;

        let x_min = 0.;
        let x_max = 10.;

        let out_of_bounds_reach = 2.0;

        let BenchPack {
            linear_interp,
            linear_clamped,
            binary_interp,
            cached_linear_interp,
            lookup_values,
        } = BenchPack::new(
            &mut rng,
            len,
            num_lookups,
            x_min,
            x_max,
            out_of_bounds_reach,
            sort_lookup_values,
        );

        benchmarks.extend(vec![
            benchmark_fn(format!("random linear clamped {len}"), move |b| {
                b.iter(move || black_box(lookup(&linear_clamped, &lookup_values)))
            }),
            benchmark_fn(format!("random linear interp {len}"), move |b| {
                b.iter(move || black_box(lookup(&linear_interp, &lookup_values)))
            }),
            benchmark_fn(format!("random binary interp {len}"), move |b| {
                b.iter(move || black_box(lookup(&binary_interp, &lookup_values)))
            }),
            benchmark_fn(
                format!("random cached linear cell interp {len}"),
                move |b| b.iter(move || black_box(lookup(&cached_linear_interp, &lookup_values))),
            ),
        ])
    }

    benchmarks
}

fn lookup_sorted() -> impl IntoBenchmarks {
    let mut rng = StdRng::seed_from_u64(10);

    let mut benchmarks = Vec::new();
    let sort_lookup_values = true;

    for len in [5, 10, 20, 50, 100, 500, 1000] {
        let num_lookups = 10_000_000;

        let x_min = 0.;
        let x_max = 10.;

        let out_of_bounds_reach = 2.0;

        let BenchPack {
            linear_interp,
            linear_clamped,
            binary_interp,
            cached_linear_interp,
            lookup_values,
        } = BenchPack::new(
            &mut rng,
            len,
            num_lookups,
            x_min,
            x_max,
            out_of_bounds_reach,
            sort_lookup_values,
        );

        benchmarks.extend(vec![
            benchmark_fn(format!("sorted linear clamped {len}"), move |b| {
                b.iter(move || black_box(lookup(&linear_clamped, &lookup_values)))
            }),
            benchmark_fn(format!("sorted linear interp {len}"), move |b| {
                b.iter(move || black_box(lookup(&linear_interp, &lookup_values)))
            }),
            benchmark_fn(format!("sorted binary interp {len}"), move |b| {
                b.iter(move || black_box(lookup(&binary_interp, &lookup_values)))
            }),
            benchmark_fn(
                format!("sorted cached_linear_cell interp {len}"),
                move |b| b.iter(move || black_box(lookup(&cached_linear_interp, &lookup_values))),
            ),
        ])
    }

    benchmarks
}

tango_benchmarks!(lookup_random(), lookup_sorted());
tango_main!();
