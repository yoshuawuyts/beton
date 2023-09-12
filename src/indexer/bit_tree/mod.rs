//! An index implemented as a tree of bits.
//!
//! # Design
//!
//! The goal of this indexer is to make it fast to find the next unoccupied
//! slot. We do this by holding two lists: one with each bit for each and every
//! index, and one tree-structure which makes it faster to identify which pages
//! are in use and which are not.
//!
//! The tree structure is divided into layers. Eeach layer can hold 32|64 items
//! of the next layer, depending on the system's bit size. At each layer a bit
//! marked as 0 indicates there is space available in the children of the next
//! layer. And a bit set to 1 means that there is no space left.
//!
//! ```text
//! 1. [1, 0, 0, 0, 0, 0, ..]
//!        |
//! 2. [0, 0, 1, 0, 0, 0, ..]
//! ```
//!
//! The way the pages are laid out in memory is in a tiered fashion. The first
//! layer exists at the start of the index. The next layer after that. And so
//! on. This makes it cheap to produce indexes

use super::utils::compute_index;
pub(crate) use into_occupied::IntoOccupied;
pub(crate) use occupied::Occupied;
pub(crate) use unoccupied::UnOccupied;

mod into_occupied;
mod occupied;
mod unoccupied;

/// An indexing structure implemented as a tree of bits.
#[derive(Debug)]
pub(crate) struct BitVec {
    tree: Vec<usize>,
    entries: Vec<usize>,
    count: usize,
}

impl Default for BitVec {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BitVec {
    /// Create an empty instance of the `index`
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Create an empty instance of the `index`
    #[allow(unused)]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        let page_capacity = compute_size(capacity);
        Self {
            tree: vec![0; page_capacity],
            entries: vec![0; capacity],
            count: 0,
        }
    }

    /// Insert an entry into the index
    #[inline]
    pub(crate) fn insert(&mut self, index: usize) {
        debug_assert!(
            index < self.capacity(),
            "Write at index {index} is out of bounds"
        );
        let (index, mask) = compute_index(index);
        self.entries[index] |= mask;
        self.count += 1;
    }

    /// Remove an entry from the index
    #[inline]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        let (index, mask) = compute_index(index);
        let ret = self.contains(index);
        match self.entries.get_mut(index) {
            Some(entry) => {
                self.count -= 1;
                *entry &= !mask;
                ret
            }
            None => false,
        }
    }

    /// Clear the entire index
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.count = 0;
        self.entries.fill(0);
    }

    /// Returns `true` if the index contains a value
    #[inline]
    pub(crate) fn contains(&self, index: usize) -> bool {
        let (index, mask) = compute_index(index);
        match self.entries.get(index) {
            Some(entry) => *entry & mask == mask,
            None => false,
        }
    }

    /// How many items are currently contained?
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.count
    }

    /// Is the structure empty?
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// What is the current capacity?
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        usize::BITS as usize * self.entries.capacity()
    }

    /// Resize the Index
    #[inline]
    pub(crate) fn resize(&mut self, new_len: usize) {
        let current_length = self.entries.len();
        self.entries.resize(new_len, 0);

        if new_len < current_length {
            self.count = self
                .entries
                .iter()
                .map(|entry| entry.count_ones() as usize)
                .sum();
        }
    }

    /// Create an iterator over the indexes occupied by items.
    #[inline]
    pub(crate) fn occupied(&self) -> Occupied {
        Occupied::new(self)
    }

    /// Create an iterator over the indexes occupied by items.
    #[inline]
    pub(crate) fn into_occupied(self) -> IntoOccupied {
        IntoOccupied::new(self)
    }

    /// Create an iterator over the indexes not occupied by items.
    #[inline]
    pub(crate) fn unoccupied(&self) -> UnOccupied {
        UnOccupied::new(self)
    }
}

#[inline]
const fn compute_depth(mut index: usize) -> usize {
    let mut depth = 0;
    loop {
        index /= usize::BITS as usize;
        match index {
            0 => break,
            _ => depth += 1,
        }
    }
    depth
}

#[inline]
const fn compute_size(index: usize) -> usize {
    let depth = compute_depth(index);
    (usize::BITS as usize).pow(depth as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(compute_size(0), 1);
        assert_eq!(compute_size(1), 1);
        assert_eq!(compute_size(2), 1);

        assert_eq!(compute_size(64 - 1), 1);
        assert_eq!(compute_size(64 + 0), 64);
    }

    #[test]
    fn depth() {
        assert_eq!(compute_depth(0), 0);
        assert_eq!(compute_depth(1), 0);
        assert_eq!(compute_depth(2), 0);

        assert_eq!(compute_depth(64 - 1), 0);
        assert_eq!(compute_depth(64 + 0), 1);
        assert_eq!(compute_depth(64 + 1), 1);
        assert_eq!(compute_depth(64 + 2), 1);

        assert_eq!(compute_depth(64usize.pow(2) - 1), 1);
        assert_eq!(compute_depth(64usize.pow(2) + 0), 2);
        assert_eq!(compute_depth(64usize.pow(2) + 1), 2);
        assert_eq!(compute_depth(64usize.pow(2) + 2), 2);
    }

    #[test]
    fn smoke() {
        let mut arr = BitVec::with_capacity(2);
        let max = arr.capacity();
        for n in 0..max {
            arr.insert(n);
            assert!(arr.contains(n));
        }

        assert_eq!(arr.len(), max);

        for n in 0..max {
            assert!(arr.contains(n));
            arr.remove(n);
            assert!(!arr.contains(n));
        }

        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn occupied() {
        let mut arr = BitVec::new();
        let max = arr.capacity();
        for n in 0..max {
            arr.insert(n);
        }

        let mut count = 0;
        for index in arr.into_occupied() {
            assert_eq!(index, count);
            count += 1;
        }
        assert_eq!(count, max);
    }
}
