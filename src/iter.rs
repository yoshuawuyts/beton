use std::mem::{self, MaybeUninit};
use std::ptr;

use crate::bit_tree::{IntoOccupied, Occupied};
use crate::Slab;

/// An owned iterator over items in the `Slab`.
#[derive(Debug)]
pub struct IntoIter<T> {
    occupied: IntoOccupied,
    entries: Vec<MaybeUninit<T>>,
}

impl<T> IntoIter<T> {
    pub(crate) fn new(slab: crate::Slab<T>) -> Self {
        // Turn the slab into a pointer so that the `Drop` constructor is no
        // longer called.
        let slab = MaybeUninit::new(slab);
        let slab = slab.as_ptr();

        // SAFETY: We're destructuring `Slab` into its components, in order to not
        // call its destructor. Instead the iterator struct now becomes
        // responsible for properly handling a `Vec<MaybeUninit<T>>`.
        unsafe {
            Self {
                occupied: ptr::read(&(*slab).index).into_occupied(),
                entries: ptr::read(&(*slab).entries),
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the item at index.
        let index = self.occupied.next()?;
        let output = mem::replace(&mut self.entries[index], MaybeUninit::uninit());

        // SAFETY: we just confirmed that there was in fact an entry at this index
        Some(unsafe { output.assume_init() })
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for index in &mut self.occupied {
            // SAFETY: we're iterating over all remaining items marked as
            // "occupied" and dropping them in-place.
            unsafe { self.entries[index].assume_init_drop() }
        }
    }
}

/// An borrowing iterator over items in the `Slab`.
#[derive(Debug)]
pub struct Iter<'a, T> {
    occupied: Occupied<'a>,
    entries: &'a Vec<MaybeUninit<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(slab: &'a Slab<T>) -> Self {
        let occupied = slab.index.occupied();
        let entries = &slab.entries;
        Self { occupied, entries }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.occupied.next()?;
        self.entries.get(usize::from(index)).map(|v| {
            // SAFETY: We just validated that the index contains a key
            // for this value, meaning we can safely assume that this
            // value is initialized.
            unsafe { v.assume_init_ref() }
        })
    }
}

/// A mutable iterator over items in the `Slab`.
#[derive(Debug)]
pub struct IterMut<'a, T> {
    occupied: Occupied<'a>,
    entries: core::slice::IterMut<'a, MaybeUninit<T>>,
    /// What index did we last index? We need this to advance the slice
    /// iterator.
    prev_index: Option<usize>,
}

impl<'a, T> IterMut<'a, T> {
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

impl<'a, T> Iterator for IterMut<'a, T> {
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
    fn into_iter() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        let key = slab.insert(2);
        slab.insert(3);
        slab.remove(key);
        let mut iter = IntoIter::new(slab);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        let key = slab.insert(2);
        slab.insert(3);
        slab.remove(key);
        let mut iter = Iter::new(&slab);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        let key = slab.insert(2);
        slab.insert(3);
        slab.remove(key);
        let mut iter = IterMut::new(&mut slab);
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
