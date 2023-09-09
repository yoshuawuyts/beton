use std::mem::MaybeUninit;

use crate::bit_tree::Occupied;
use crate::{Key, Slab};

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
    type Item = (Key, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.occupied.next()?;
        self.entries.get(usize::from(index)).map(|v| {
            // SAFETY: We just validated that the index contains a key
            // for this value, meaning we can safely assume that this
            // value is initialized.
            (index.into(), unsafe { v.assume_init_ref() })
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
        let mut iter = Iter::new(&slab);
        assert_eq!(iter.next(), Some((0.into(), &1)));
        assert_eq!(iter.next(), Some((2.into(), &3)));
        assert_eq!(iter.next(), None);
    }
}
