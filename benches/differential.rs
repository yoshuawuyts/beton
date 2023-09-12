use std::{borrow::BorrowMut, collections::BTreeSet};

use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use heckcheck::prelude::*;

struct Slab {
    index: BTreeSet<usize>,
    slab: slab::Slab<usize>,
}

impl Slab {
    fn new() -> Self {
        Self {
            index: BTreeSet::new(),
            slab: slab::Slab::new(),
        }
    }

    fn with_capacity(cap: usize) -> Self {
        Self {
            index: BTreeSet::new(),
            slab: slab::Slab::with_capacity(cap),
        }
    }

    fn insert(&mut self, value: usize) {
        self.index.insert(value);
        self.slab.insert(value);
    }

    fn contains(&self, value: usize) -> bool {
        self.index.contains(&value)
    }

    fn get(&self, value: usize) -> Option<&usize> {
        self.index.get(&value)
    }

    fn remove(&mut self, value: usize) {
        self.index.remove(&value);
        self.slab.try_remove(value);
    }

    fn clear(&mut self) {
        self.index.clear();
        self.slab.clear();
    }

    fn iter(&mut self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.index
            .iter()
            .map(|key| (*key, *self.slab.borrow_mut().get(*key).unwrap()))
    }
}

fn insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");
    for i in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::new("beton", i), i, |b, i| {
            let setup = || beton::Slab::with_capacity(*i);
            let routine = |mut slab: beton::Slab<usize>| {
                black_box({
                    for n in 0..*i {
                        slab.insert(n);
                    }
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });

        group.bench_with_input(BenchmarkId::new("slab (without index)", i), i, |b, i| {
            let setup = || slab::Slab::with_capacity(*i);
            let routine = |mut slab: slab::Slab<usize>| {
                black_box({
                    for n in 0..*i {
                        slab.insert(n);
                    }
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });

        group.bench_with_input(BenchmarkId::new("slab (with index)", i), i, |b, i| {
            let setup = || Slab::with_capacity(*i);
            let routine = |mut slab: Slab| {
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

fn mutate(c: &mut Criterion) {
    let mut group = c.benchmark_group("mutate");
    let seed = fastrand::u64(1024..2048);

    /// A single operation we can apply
    #[derive(Arbitrary, Debug)]
    enum Operation {
        Insert(usize),
        Fetch(usize),
        Remove(usize),
        Contains(usize),
        Clear,
    }

    group.bench_function(BenchmarkId::new("beton", "mutate"), |b| {
        let setup = || beton::Slab::new();
        let routine = |mut slab: beton::Slab<usize>| {
            black_box({
                let mut checker = heckcheck::HeckCheck::from_seed(seed);
                checker.check(|operations: Vec<Operation>| {
                    for operation in operations {
                        match operation {
                            Operation::Insert(value) => {
                                let _key = slab.insert(value);
                            }
                            Operation::Fetch(index) => {
                                let _output = slab.get(index.into());
                            }
                            Operation::Remove(index) => {
                                let _output = slab.remove(index.into());
                            }
                            Operation::Contains(index) => {
                                let _contains = slab.contains_key(index.into());
                            }
                            Operation::Clear => {
                                slab.clear();
                            }
                        }
                    }
                    Ok(())
                });
            });
        };
        b.iter_batched(setup, routine, BatchSize::SmallInput)
    });

    group.bench_function(BenchmarkId::new("slab (without index)", "mutate"), |b| {
        let setup = || Slab::new();
        let routine = |mut slab: Slab| {
            black_box({
                let mut checker = heckcheck::HeckCheck::from_seed(seed);
                checker.check(|operations: Vec<Operation>| {
                    for operation in operations {
                        match operation {
                            Operation::Insert(value) => {
                                let _key = slab.insert(value);
                            }
                            Operation::Fetch(index) => {
                                let _output = slab.get(index);
                            }
                            Operation::Remove(index) => {
                                let _output = slab.remove(index);
                            }
                            Operation::Contains(index) => {
                                let _contains = slab.contains(index);
                            }
                            Operation::Clear => {
                                slab.clear();
                            }
                        }
                    }
                    Ok(())
                });
            });
        };
        b.iter_batched(setup, routine, BatchSize::SmallInput)
    });
    group.bench_function(BenchmarkId::new("slab (with index)", "mutate"), |b| {
        let setup = || slab::Slab::new();
        let routine = |mut slab: slab::Slab<usize>| {
            black_box({
                let mut checker = heckcheck::HeckCheck::from_seed(seed);
                checker.check(|operations: Vec<Operation>| {
                    for operation in operations {
                        match operation {
                            Operation::Insert(value) => {
                                let _key = slab.insert(value);
                            }
                            Operation::Fetch(index) => {
                                let _output = slab.get(index);
                            }
                            Operation::Remove(index) => {
                                let _output = slab.try_remove(index);
                            }
                            Operation::Contains(index) => {
                                let _contains = slab.contains(index);
                            }
                            Operation::Clear => {
                                slab.clear();
                            }
                        }
                    }
                    Ok(())
                });
            });
        };
        b.iter_batched(setup, routine, BatchSize::SmallInput)
    });
    group.finish();
}

fn iterate(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterate");
    for i in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::new("beton", i), i, |b, i| {
            let setup = || {
                let mut slab = beton::Slab::with_capacity(*i);
                let mut total = 0;
                for n in 0..*i {
                    total += n;
                    slab.insert(n);
                }
                (slab, total)
            };
            let routine = |(slab, total): (beton::Slab<usize>, usize)| {
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

        group.bench_with_input(BenchmarkId::new("slab (without index)", i), i, |b, i| {
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
                    for (_, n) in slab.iter() {
                        sum += n;
                    }
                    assert_eq!(sum, total);
                });
            };
            b.iter_batched(setup, routine, BatchSize::SmallInput)
        });

        group.bench_with_input(BenchmarkId::new("slab (with index)", i), i, |b, i| {
            let setup = || {
                let mut slab = Slab::with_capacity(*i);
                let mut total = 0;
                for n in 0..*i {
                    total += n;
                    slab.insert(n);
                }
                (slab, total)
            };
            let routine = |(mut slab, total): (Slab, usize)| {
                black_box({
                    let mut sum = 0;
                    for (_, n) in slab.iter() {
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

criterion_group!(operations_with_index, insert, mutate, iterate);
criterion_main!(operations_with_index);
