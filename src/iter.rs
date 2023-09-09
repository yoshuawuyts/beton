use std::mem::{self, MaybeUninit};
use std::ptr;

use crate::bit_tree::{IntoOccupied, Occupied};
use crate::Slab;

/// An owned iterator over items in the `Slab`.
pub struct IntoIter<T> {
    occupied: IntoOccupied,
    entries: Vec<MaybeUninit<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntoIter")
            .field("occupied", &self.occupied)
            .field("entries", &self.entries)
            .finish()
    }
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

// /// A mutable iterator over items in the `Slab`.
// #[derive(Debug)]
// pub struct IterMut<'a, T> {
//     cursor: usize,
//     index: &'a BitTree,
//     iter: core::slice::IterMut<'a, MaybeUninit<T>>,
// }

// impl<'a, T> IterMut<'a, T> {
//     pub fn new(slab: &'a mut Slab<T>) -> Self {
//         Self {
//             cursor: 0,
//             index: &slab.index,
//             iter: slab.entries.iter_mut(),
//         }
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let prev_cursor = self.cursor;
//         let cursor = self.index.next_occupied(self.cursor)?;
//         self.cursor = cursor + 1;

//         let skip = match cursor {
//             0 => 0,
//             cursor => cursor - prev_cursor - 1,
//         };

//         advance_by(&mut self.iter, dbg!(skip));
//         self.iter.next().map(|t| unsafe { t.assume_init_mut() })
//     }
// }

// // TODO: Waiting for `Iterator::advance_by` to be stabilized
// // https://github.com/rust-lang/rust/issues/77404
// fn advance_by(iter: &mut impl Iterator, n: usize) {
//     for _ in 0..n {
//         iter.next();
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn iter_mut() {
    //     let mut slab = crate::Slab::new();
    //     slab.insert(1);
    //     slab.insert(2);
    //     let mut iter = IterMut::new(&mut slab);
    //     assert_eq!(dbg!(iter.next()), Some(&mut 1));
    //     assert_eq!(dbg!(iter.next()), Some(&mut 2));
    //     assert_eq!(dbg!(iter.next()), None);
    // }

    #[test]
    fn into_iter() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        slab.insert(2);
        let mut iter = IntoIter::new(slab);
        assert_eq!(dbg!(iter.next()), Some(1));
        assert_eq!(dbg!(iter.next()), Some(2));
        assert_eq!(dbg!(iter.next()), None);
    }

    #[test]
    fn iter() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        slab.insert(2);
        let mut iter = Iter::new(&slab);
        assert_eq!(dbg!(iter.next()), Some(&1));
        assert_eq!(dbg!(iter.next()), Some(&2));
        assert_eq!(dbg!(iter.next()), None);
    }
}
