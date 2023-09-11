// use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
// use heckcheck::prelude::*;

// fn insert(c: &mut Criterion) {
//     let mut group = c.benchmark_group("insert");
//     for i in [10, 100, 1_000, 10_000].iter() {
//         group.bench_with_input(BenchmarkId::new("beton", i), i, |b, i| {
//             let setup = || tree_slab::Slab::with_capacity(*i);
//             let routine = |mut slab: tree_slab::Slab<usize>| {
//                 black_box({
//                     for n in 0..*i {
//                         slab.insert(n);
//                     }
//                 });
//             };
//             b.iter_batched(setup, routine, BatchSize::SmallInput)
//         });

//         group.bench_with_input(BenchmarkId::new("slab", i), i, |b, i| {
//             let setup = || slab::Slab::with_capacity(*i);
//             let routine = |mut slab: slab::Slab<usize>| {
//                 black_box({
//                     for n in 0..*i {
//                         slab.insert(n);
//                     }
//                 });
//             };
//             b.iter_batched(setup, routine, BatchSize::SmallInput)
//         });
//     }
//     group.finish();
// }

// fn mutate(c: &mut Criterion) {
//     let mut group = c.benchmark_group("mutate");
//     let seed = fastrand::u64(1024..2048);

//     /// A single operation we can apply
//     #[derive(Arbitrary, Debug)]
//     enum Operation {
//         Insert(usize),
//         Fetch(usize),
//         Remove(usize),
//         Contains(usize),
//         Clear,
//         Reserve(usize),
//     }

//     group.bench_function(BenchmarkId::new("beton", "main"), |b| {
//         let setup = || tree_slab::Slab::new();
//         let routine = |mut slab: tree_slab::Slab<usize>| {
//             black_box({
//                 let mut checker = heckcheck::HeckCheck::from_seed(seed);
//                 checker.check(|operations: Vec<Operation>| {
//                     for operation in operations {
//                         match operation {
//                             Operation::Insert(value) => {
//                                 let _key = slab.insert(value);
//                             }
//                             Operation::Fetch(index) => {
//                                 let _output = slab.get(index.into());
//                             }
//                             Operation::Remove(index) => {
//                                 let _output = slab.remove(index.into());
//                             }
//                             Operation::Contains(index) => {
//                                 let _contains = slab.contains_key(index.into());
//                             }
//                             Operation::Clear => {
//                                 slab.clear();
//                             }
//                             Operation::Reserve(capacity) => {
//                                 // NOTE: very big allocations make this slow, so we ensure
//                                 // they're always a little smaller
//                                 //
//                                 // FIXME: in the proper impl of BitTree, we should be able
//                                 // to a `memset(3)` call.
//                                 let capacity = capacity % 1024;
//                                 slab.reserve(capacity);
//                             }
//                         }
//                     }
//                     Ok(())
//                 });
//             });
//         };
//         b.iter_batched(setup, routine, BatchSize::SmallInput)
//     });

//     group.bench_function(BenchmarkId::new("slab", "main"), |b| {
//         let setup = || slab::Slab::new();
//         let routine = |mut slab: slab::Slab<usize>| {
//             black_box({
//                 let mut checker = heckcheck::HeckCheck::from_seed(seed);
//                 checker.check(|operations: Vec<Operation>| {
//                     for operation in operations {
//                         match operation {
//                             Operation::Insert(value) => {
//                                 let _key = slab.insert(value);
//                             }
//                             Operation::Fetch(index) => {
//                                 let _output = slab.get(index);
//                             }
//                             Operation::Remove(index) => {
//                                 let _output = slab.try_remove(index);
//                             }
//                             Operation::Contains(index) => {
//                                 let _contains = slab.contains(index);
//                             }
//                             Operation::Clear => {
//                                 slab.clear();
//                             }
//                             Operation::Reserve(capacity) => {
//                                 // NOTE: very big allocations make this slow, so we ensure
//                                 // they're always a little smaller
//                                 let capacity = capacity % 1024;
//                                 slab.reserve(capacity);
//                             }
//                         }
//                     }
//                     Ok(())
//                 });
//             });
//         };
//         b.iter_batched(setup, routine, BatchSize::SmallInput)
//     });
//     group.finish();
// }

// fn iterate(c: &mut Criterion) {
//     let mut group = c.benchmark_group("iterate");
//     for i in [10, 100, 1_000, 10_000].iter() {
//         group.bench_with_input(BenchmarkId::new("beton", i), i, |b, i| {
//             let setup = || {
//                 let mut slab = tree_slab::Slab::with_capacity(*i);
//                 let mut total = 0;
//                 for n in 0..*i {
//                     total += n;
//                     slab.insert(n);
//                 }
//                 (slab, total)
//             };
//             let routine = |(slab, total): (tree_slab::Slab<usize>, usize)| {
//                 black_box({
//                     let mut sum = 0;
//                     for (_, n) in slab {
//                         sum += n;
//                     }
//                     assert_eq!(sum, total);
//                 });
//             };
//             b.iter_batched(setup, routine, BatchSize::SmallInput)
//         });

//         group.bench_with_input(BenchmarkId::new("slab", i), i, |b, i| {
//             let setup = || {
//                 let mut slab = slab::Slab::with_capacity(*i);
//                 let mut total = 0;
//                 for n in 0..*i {
//                     total += n;
//                     slab.insert(n);
//                 }
//                 (slab, total)
//             };
//             let routine = |(slab, total): (slab::Slab<usize>, usize)| {
//                 black_box({
//                     let mut sum = 0;
//                     for (_, n) in slab {
//                         sum += n;
//                     }
//                     assert_eq!(sum, total);
//                 });
//             };
//             b.iter_batched(setup, routine, BatchSize::SmallInput)
//         });
//     }
//     group.finish();
// }

// criterion_group!(operations, insert, mutate, iterate);
// criterion_main!(operations);

fn main() {}
