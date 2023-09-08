use tree_slab::Slab;

/// Test we don't go out of bounds when attempting to remove a key which is
/// currently not in range.
#[test]
fn remove_zero() {
    let mut subject: Slab<usize> = Slab::new();
    subject.remove(0.into());
}
