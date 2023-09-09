use std::mem::{self, MaybeUninit};

use crate::{bit_tree::BitTree, Slab};

/// An owned iterator over items in the `Slab`.
#[derive(Debug)]
pub struct IntoIter<T> {
    cursor: usize,
    slab: Slab<T>,
}

impl<T> IntoIter<T> {
    pub(crate) fn new(inner: crate::Slab<T>) -> Self {
        Self {
            slab: inner,
            cursor: 0,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = match self.slab.index.next_occupied(self.cursor) {
            Some(index) => index,
            None => return None,
        };

        let mut output = MaybeUninit::uninit();
        mem::swap(&mut self.slab.entries[self.cursor], &mut output);
        self.slab.index.remove(self.cursor);

        // SAFETY: we just confirmed that there was in fact an entry at this index
        Some(unsafe { output.assume_init() })
    }
}

/// An borrowing iterator over items in the `Slab`.
#[derive(Debug)]
pub struct Iter<'a, T> {
    cursor: usize,
    slab: &'a crate::Slab<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.slab.index.next_occupied(self.cursor)?;
        self.cursor = index;
        self.slab.get(index.into())
    }
}

/// A mutable iterator over items in the `Slab`.
#[derive(Debug)]
pub struct IterMut<'a, T> {
    cursor: usize,
    index: &'a BitTree,
    iter: core::slice::IterMut<'a, MaybeUninit<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(slab: &'a mut Slab<T>) -> Self {
        Self {
            cursor: 0,
            index: &slab.index,
            iter: slab.entries.iter_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_cursor = self.cursor;
        let cursor = self.index.next_occupied(self.cursor)?;
        self.cursor = cursor + 1;

        let skip = match cursor {
            0 => 0,
            cursor => cursor - prev_cursor - 1,
        };

        advance_by(&mut self.iter, dbg!(skip));
        self.iter.next().map(|t| unsafe { t.assume_init_mut() })
    }
}

// TODO: Waiting for `Iterator::advance_by` to be stabilized
// https://github.com/rust-lang/rust/issues/77404
fn advance_by(iter: &mut impl Iterator, n: usize) {
    for _ in 0..n {
        iter.next();
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn iter_mut() {
//         let mut slab = crate::Slab::new();
//         slab.insert(1);
//         slab.insert(2);
//         let mut iter = IterMut::new(&mut slab);
//         assert_eq!(dbg!(iter.next()), Some(&mut 1));
//         assert_eq!(dbg!(iter.next()), Some(&mut 2));
//         assert_eq!(dbg!(iter.next()), None);
//     }
// }
