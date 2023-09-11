use std::mem::MaybeUninit;

use crate::indexer::Occupied;
use crate::Slab;

/// An borrowing iterator over items in the `Slab`.
#[derive(Debug)]
pub struct Values<'a, T> {
    occupied: Occupied<'a>,
    entries: &'a Vec<MaybeUninit<T>>,
}

impl<'a, T> Values<'a, T> {
    pub(crate) fn new(slab: &'a Slab<T>) -> Self {
        let occupied = slab.index.occupied();
        let entries = &slab.entries;
        Self { occupied, entries }
    }
}

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.occupied.next()?;
        self.entries.get(index).map(|v| {
            // SAFETY: We just validated that the index contains a key
            // for this value, meaning we can safely assume that this
            // value is initialized.
            unsafe { v.assume_init_ref() }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        let key = slab.insert(2);
        slab.insert(3);
        slab.remove(key);
        let mut iter = Values::new(&slab);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
