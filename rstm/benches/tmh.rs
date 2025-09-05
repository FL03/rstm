/*
    Appellation: default <module>
    Contrib: @FL03
*/
use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion};
use rstm::prelude::{Program, TMH};
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
            (0, 0) -> Right(1, 0);
            (0, 1) -> Left(-1, 1);
            (1, 0) -> Right(1, 1);
            (1, 1) -> Right(-1, 0);
            (-1, 0) -> Left(<isize>::MAX, 0);
            (-1, 1) -> Left(0, 1);
        };
    };
}

fn setup_tmh() -> TMH<isize, usize> {
    // define some input for the machine
    let input = vec![0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0];
    // initialize the state of the machine
    let initial_state: isize = 0;
    // create a new instance of the machine
    TMH::new(initial_state, input)
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
                let mut engine = tmh.execute(PROGRAM.clone());
                black_box(engine.step().expect("should step"));
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
