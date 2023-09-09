use std::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
};

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
    ptr: *mut MaybeUninit<T>,
    end: *mut MaybeUninit<T>,
    // not sure if this should be &mut T or &mut MaybeUninit<T>
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(slab: &'a mut Slab<T>) -> Self {
        assert_ne!(mem::size_of::<T>(), 0);
        let len = slab.entries.len();
        let ptr = slab.entries.as_mut_ptr();
        Self {
            ptr,
            cursor: 0,
            index: &slab.index,
            end: unsafe { ptr.add(len) },
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        assert_ne!(mem::size_of::<T>(), 0);
        if self.ptr == self.end {
            return None;
        }

        // snapshot of the current state
        let old = self.ptr;
        let cursor = self.cursor;

        // setup the state for the next iteration
        let index = self.index.next_occupied(self.cursor)?;
        self.cursor = index;

        // access the current element
        let offset = index - cursor;
        self.ptr = unsafe { self.ptr.add(offset) };
        unsafe {
            let uninit = &mut *old;
            Some(uninit.assume_init_mut())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn iter_mut() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        slab.insert(2);
        let mut iter = IterMut::new(&mut slab);
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);
    }
}
