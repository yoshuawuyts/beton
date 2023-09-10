use std::mem::{self, MaybeUninit};
use std::ptr;

use crate::indexer::IntoOccupied;
use crate::Key;

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
    type Item = (Key, T);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the item at index.
        let index = self.occupied.next()?;
        let output = mem::replace(&mut self.entries[index], MaybeUninit::uninit());

        // SAFETY: we just confirmed that there was in fact an entry at this index
        Some((index.into(), unsafe { output.assume_init() }))
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
        assert_eq!(iter.next(), Some((0.into(), 1)));
        assert_eq!(iter.next(), Some((2.into(), 3)));
        assert_eq!(iter.next(), None);
    }
}
