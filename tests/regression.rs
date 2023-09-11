use beton::Slab;

/// Test we don't go out of bounds when attempting to remove a key which is
/// currently not in range.
#[test]
fn remove_zero() {
    let mut subject: Slab<usize> = Slab::new();
    subject.remove(0.into());
}

/// Test we don't go out of bounds when attempting to remove a key which
/// exceeds the current bitset.
#[test]
fn remove_out_of_bounds() {
    let mut subject: Slab<usize> = Slab::new();
    subject.remove(4215.into());
}
