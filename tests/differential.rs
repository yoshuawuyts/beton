use heckcheck::prelude::*;

/// A single operation we can apply
#[derive(Arbitrary, Debug)]
enum Operation {
    Insert(usize),
    Fetch(usize),
    Remove(usize),
    Contains(usize),
    Clear,
    Reserve(usize),
}

#[test]
fn differential_slab() {
    heckcheck::check(|operations: Vec<Operation>| {
        // Setup both our subject and the oracle
        let mut oracle = slab::Slab::new();
        let mut subject = beton::Slab::new();

        // Apply the same operations in-order to both the subject and the oracle
        // comparing outputs whenever we get any.
        for operation in operations {
            match operation {
                Operation::Insert(value) => {
                    let key1 = oracle.insert(value);
                    let key2 = subject.insert(value);
                    assert_eq!(key1, key2.into());
                }
                Operation::Fetch(index) => {
                    assert_eq!(oracle.get(index), subject.get(index.into()));
                }
                Operation::Remove(index) => {
                    assert_eq!(oracle.try_remove(index), subject.remove(index.into()));
                }
                Operation::Contains(index) => {
                    assert_eq!(oracle.contains(index), subject.contains_key(index.into()));
                }
                Operation::Clear => {
                    oracle.clear();
                    subject.clear();
                }
                Operation::Reserve(capacity) => {
                    // NOTE: very big allocations make this slow, so we ensure
                    // they're always a little smaller
                    //
                    // FIXME: in the proper impl of BitTree, we should be able
                    // to a `memset(3)` call.
                    let capacity = capacity % 1024;

                    oracle.reserve(capacity);
                    subject.reserve(capacity);
                }
            }
        }
        Ok(())
    });
}
