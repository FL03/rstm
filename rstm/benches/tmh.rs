/*
    Appellation: tmh <module>
    Created At: 2026.01.15:10:49:11
    Contrib: @FL03
*/
use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion};
use rstm::prelude::{Head, MovingHead, Program};
use std::time::Duration;

const SAMPLES: usize = 50;
/// the default number of iterations to benchmark a method for
const N: usize = 20;
/// the default number of seconds a benchmark should complete in
const DEFAULT_DURATION_SECS: u64 = 10;

lazy_static::lazy_static! {
    static ref PROGRAM: Program<isize, usize> = rstm::program! {
        #[default_state(0)]
        rules: {
            (0, 0) -> Right(1, 0),
            (0, 1) -> Left(-1, 1),
            (1, 0) -> Right(1, 1),
            (1, 1) -> Right(-1, 0),
            (-1, 0) -> Left(<isize>::MAX, 0),
            (-1, 1) -> Left(0, 1),
        };
    };
}

fn setup_tmh() -> MovingHead<isize, usize> {
    // define some input for the machine
    let input = [0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0];
    // create a new instance of the machine
    let mut engine = Head::new(0, 0).load(PROGRAM.clone());
    // load the input into the machine tape
    engine.extend_tape(input);
    engine
}
fn bench_tmh(c: &mut Criterion) {
    let measure_for = Duration::from_secs(DEFAULT_DURATION_SECS);
    // create a benchmark group for the TMH machine
    let mut group = c.benchmark_group("TMH");
    group.measurement_time(measure_for);
    group.sample_size(SAMPLES);

    // Benchmark a single step
    group.bench_function(BenchmarkId::new("step", N), |b| {
        b.iter_batched(
            setup_tmh,
            |mut tmh| {
                black_box(tmh.step().expect("should step"));
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}
// initialize the benchmark group
criterion::criterion_group! {
    benches,
    bench_tmh,
}
// This macro expands to a function named `benches`, which uses the given config
criterion::criterion_main!(benches);
