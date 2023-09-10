use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    for i in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::new("beton", i), i, |b, i| {
            let setup = || tree_slab::Slab::new();
            let routine = |mut slab: tree_slab::Slab<usize>| {
                black_box({
                    for n in 0..*i {
                        slab.insert(n);
                    }
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });

        group.bench_with_input(BenchmarkId::new("slab", i), i, |b, i| {
            let setup = || slab::Slab::new();
            let routine = |mut slab: slab::Slab<usize>| {
                black_box({
                    for n in 0..*i {
                        slab.insert(n);
                    }
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });
    }
    group.finish();
}

fn iterate(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterate");
    for i in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::new("beton", i), i, |b, i| {
            let setup = || {
                let mut slab = tree_slab::Slab::with_capacity(*i);
                let mut total = 0;
                for n in 0..*i {
                    total += n;
                    slab.insert(n);
                }
                (slab, total)
            };
            let routine = |(slab, total): (tree_slab::Slab<usize>, usize)| {
                black_box({
                    let mut sum = 0;
                    for (_, n) in slab {
                        sum += n;
                    }
                    assert_eq!(sum, total);
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });

        group.bench_with_input(BenchmarkId::new("slab", i), i, |b, i| {
            let setup = || {
                let mut slab = slab::Slab::with_capacity(*i);
                let mut total = 0;
                for n in 0..*i {
                    total += n;
                    slab.insert(n);
                }
                (slab, total)
            };
            let routine = |(slab, total): (slab::Slab<usize>, usize)| {
                black_box({
                    let mut sum = 0;
                    for (_, n) in slab {
                        sum += n;
                    }
                    assert_eq!(sum, total);
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });
    }
    group.finish();
}

criterion_group!(operations, insert, iterate);
criterion_main!(operations);
