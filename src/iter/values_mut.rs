use std::mem::MaybeUninit;

use crate::bit_tree::Occupied;
use crate::Slab;

/// A mutable iterator over items in the `Slab`.
#[derive(Debug)]
pub struct ValuesMut<'a, T> {
    occupied: Occupied<'a>,
    entries: core::slice::IterMut<'a, MaybeUninit<T>>,
    /// What index did we last index? We need this to advance the slice
    /// iterator.
    prev_index: Option<usize>,
}

impl<'a, T> ValuesMut<'a, T> {
    pub(crate) fn new(slab: &'a mut Slab<T>) -> Self {
        let occupied = slab.index.occupied();
        let entries = slab.entries.iter_mut();
        Self {
            occupied,
            entries,
            prev_index: None,
        }
    }
}

impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next index and update all cursors
        let index = self.occupied.next()?;
        let skip = match self.prev_index {
            None => 0,
            Some(prev_index) => index - prev_index - 1,
        };
        self.prev_index = Some(index);
        advance_by(&mut self.entries, skip);

        // SAFETY: we just confirmed that there was in fact an entry at this index
        self.entries.next().map(|t| unsafe { t.assume_init_mut() })
    }
}

// TODO: Waiting for `Iterator::advance_by` to be stabilized
// https://github.com/rust-lang/rust/issues/77404
fn advance_by(iter: &mut impl Iterator, n: usize) {
    for _ in 0..n {
        iter.next();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_mut() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        let key = slab.insert(2);
        slab.insert(3);
        slab.remove(key);
        let mut iter = ValuesMut::new(&mut slab);
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
