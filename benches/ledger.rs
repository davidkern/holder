use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    measurement::Measurement,
    BatchSize,
    BenchmarkGroup,
    BenchmarkId,
    Criterion,
    Throughput,
};
use holder::{Ledger, Direction};
use std::num::NonZeroUsize;

// comprehensive:
//const INCREMENTAL_ALLOCATION_COUNTS: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];

// quick(er):
const INCREMENTAL_ALLOCATION_COUNTS: [usize; 2] = [1_000, 10_000];
const INCREMENTAL_ALLOCATION_CAPACITY: usize = 10_000_000; // allocation never fails

/// Allocate a `count` of elements one at a time.
pub fn incremental_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental-allocation");
    for count in INCREMENTAL_ALLOCATION_COUNTS.iter() {
        group.throughput(Throughput::Elements(*count as u64));
        bench_incremental_allocation(
            &mut group,
            *count,
            "limit-forward",
            Limit::new(),
            |x| { black_box(x.forward()); }
        );

        group.throughput(Throughput::Elements(*count as u64));
        bench_incremental_allocation(
            &mut group,
            *count,
            "limit-reverse",
            Limit::new(),
            |x| { black_box(x.reverse()); }
        );

        group.throughput(Throughput::Elements(*count as u64));
        bench_incremental_allocation(
            &mut group,
            *count,
            "sequential-forward",
            Ledger::new(INCREMENTAL_ALLOCATION_CAPACITY),
            |x| { x.allocate(black_box(Direction::Forward), NonZeroUsize::new(1).unwrap()).unwrap(); }
        );

        group.throughput(Throughput::Elements(*count as u64));
        bench_incremental_allocation(
            &mut group,
            *count,
            "sequential-reverse",
            Ledger::new(INCREMENTAL_ALLOCATION_CAPACITY),
            |x| { x.allocate(black_box(Direction::Reverse), NonZeroUsize::new(1).unwrap()).unwrap(); }
        );
    }

    group.finish();
}

criterion_group!(benches, incremental_allocation);
criterion_main!(benches);

fn bench_incremental_allocation<M: Measurement, S: Into<String>, T: Clone, F: Fn(&mut T)>(
    group: &mut BenchmarkGroup<M>,
    count: usize,
    name: S,
    initial: T,
    func: F)
{
    group.bench_with_input(
        BenchmarkId::new(name, count),
        &count,
        |b, &count| {
            b.iter_batched_ref(
                || initial.clone(),
                |subject| {
                    for _ in 0..count {
                        (func)(subject);
                    }
                },
                BatchSize::SmallInput);
        });
}

/// Limit case: increment/decrement indexes with no checking
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Limit {
    forward: usize,
    _padding: u128, // ensure forward and reverse don't share a cacheline
    reverse: usize,
}

impl Limit {
    pub fn new() -> Self {
        Self {
            forward: 0,
            _padding: 0,
            reverse: usize::MAX,
        }
    }

    #[inline]
    pub fn forward(&mut self) -> usize {
        let value = self.forward;
        self.forward += 1;
        self.forward
    }

    #[inline]
    pub fn reverse(&mut self) -> usize {
        self.reverse -= 1;
        self.reverse
    }
}