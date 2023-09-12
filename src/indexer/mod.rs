mod bit_array;
mod bit_tree;
mod bit_vec;

use bit_array::BitArray;
use bit_vec::BitVec;

/// How many bits should our in-line strucutre hold?
const CAPACITY: usize = 2;

#[derive(Debug)]
enum Inner {
    BitVec(BitVec),
    BitArray(BitArray<CAPACITY>),
}

/// An indexing structure with variable backends.
#[derive(Debug)]
pub(crate) struct Indexer {
    inner: Inner,
}

impl Default for Indexer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Indexer {
    /// Create an empty instance of the `index`
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            inner: Inner::BitArray(BitArray::new()),
        }
    }

    /// Initialize the index with capacity
    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        if capacity < (u64::BITS as usize * CAPACITY) {
            Self::new()
        } else {
            Self {
                inner: Inner::BitVec(BitVec::with_capacity(capacity)),
            }
        }
    }

    /// Insert an entry into the index
    #[inline]
    pub(crate) fn insert(&mut self, index: usize) {
        match self.inner {
            Inner::BitVec(ref mut vec) => vec.insert(index),
            Inner::BitArray(ref mut vec) => {
                // Bitvec has a fixed capacity. If we're going to write out of
                // bounds we should switch over to a `BitVec` instead.
                let capacity = vec.capacity();
                if index >= capacity {
                    self.resize(capacity * 2);
                    self.insert(index);
                } else {
                    vec.insert(index);
                }
            }
        }
    }

    /// Remove an entry from the index
    #[inline]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        match self.inner {
            Inner::BitVec(ref mut vec) => vec.remove(index),
            Inner::BitArray(ref mut vec) => vec.remove(index),
        }
    }

    /// Clear the entire index
    #[inline]
    pub(crate) fn clear(&mut self) {
        match self.inner {
            Inner::BitVec(ref mut vec) => vec.clear(),
            Inner::BitArray(ref mut vec) => vec.clear(),
        }
    }

    /// Returns `true` if the index contains a value
    #[inline]
    pub(crate) fn contains(&self, index: usize) -> bool {
        match self.inner {
            Inner::BitVec(ref vec) => vec.contains(index),
            Inner::BitArray(ref vec) => vec.contains(index),
        }
    }

    /// How many items are currently contained?
    #[inline]
    pub(crate) fn len(&self) -> usize {
        match self.inner {
            Inner::BitVec(ref vec) => vec.len(),
            Inner::BitArray(ref vec) => vec.len(),
        }
    }

    /// Is the structure empty?
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        match self.inner {
            Inner::BitVec(ref vec) => vec.is_empty(),
            Inner::BitArray(ref vec) => vec.is_empty(),
        }
    }

    /// What is the current capacity?
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        match &self.inner {
            Inner::BitVec(vec) => vec.capacity(),
            Inner::BitArray(vec) => vec.capacity(),
        }
    }

    /// Resize the Index
    #[inline]
    pub(crate) fn resize(&mut self, new_len: usize) {
        match &mut self.inner {
            Inner::BitVec(vec) => vec.resize(new_len),
            Inner::BitArray(arr) => {
                if new_len > arr.capacity() {
                    let mut bit_vec = BitVec::with_capacity(new_len);
                    for index in arr.occupied() {
                        bit_vec.insert(index);
                    }
                    self.inner = Inner::BitVec(bit_vec);
                }
            }
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

#[derive(Debug)]
enum OccupiedInner<'a> {
    BitVec(bit_vec::Occupied<'a>),
    BitArray(bit_array::Occupied<'a, CAPACITY>),
}

#[derive(Debug)]
pub(crate) struct Occupied<'a>(OccupiedInner<'a>);

impl<'a> Occupied<'a> {
    #[inline]
    fn new(bit_tree: &'a Indexer) -> Self {
        match bit_tree.inner {
            Inner::BitVec(ref vec) => {
                let occupied = vec.occupied();
                Self(OccupiedInner::BitVec(occupied))
            }
            Inner::BitArray(ref vec) => {
                let occupied = vec.occupied();
                Self(OccupiedInner::BitArray(occupied))
            }
        }
    }
}

impl<'a> Iterator for Occupied<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OccupiedInner::BitVec(ref mut vec) => vec.next(),
            OccupiedInner::BitArray(ref mut vec) => vec.next(),
        }
    }
}

#[derive(Debug)]
enum UnOccupiedInner<'a> {
    BitVec(bit_vec::UnOccupied<'a>),
    BitArray(bit_array::UnOccupied<'a, CAPACITY>),
}

#[derive(Debug)]
pub(crate) struct UnOccupied<'a>(UnOccupiedInner<'a>);

impl<'a> UnOccupied<'a> {
    #[inline]
    fn new(bit_tree: &'a Indexer) -> Self {
        match bit_tree.inner {
            Inner::BitVec(ref vec) => {
                let unoccupied = vec.unoccupied();
                Self(UnOccupiedInner::BitVec(unoccupied))
            }
            Inner::BitArray(ref vec) => {
                let unoccupied = vec.unoccupied();
                Self(UnOccupiedInner::BitArray(unoccupied))
            }
        }
    }
}

impl<'a> Iterator for UnOccupied<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            UnOccupiedInner::BitVec(ref mut vec) => vec.next(),
            UnOccupiedInner::BitArray(ref mut vec) => match vec.next() {
                Some(index) => Some(index),
                None => Some(u64::BITS as usize * CAPACITY),
            },
        }
    }
}

#[derive(Debug)]
enum IntoOccupiedInner {
    BitVec(bit_vec::IntoOccupied),
    BitArray(bit_array::IntoOccupied<CAPACITY>),
}

#[derive(Debug)]
pub(crate) struct IntoOccupied(IntoOccupiedInner);

impl IntoOccupied {
    #[inline]
    fn new(bit_tree: Indexer) -> Self {
        match bit_tree.inner {
            Inner::BitVec(vec) => {
                let occupied = vec.into_occupied();
                Self(IntoOccupiedInner::BitVec(occupied))
            }
            Inner::BitArray(vec) => {
                let occupied = vec.into_occupied();
                Self(IntoOccupiedInner::BitArray(occupied))
            }
        }
    }
}

impl Iterator for IntoOccupied {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IntoOccupiedInner::BitVec(ref mut vec) => vec.next(),
            IntoOccupiedInner::BitArray(ref mut vec) => vec.next(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let max = 256;
        let mut indexer = Indexer::new();

        for n in 0..max {
            indexer.insert(n);
            assert!(indexer.contains(n));
        }

        assert_eq!(indexer.len(), max);

        for n in 0..max {
            assert!(indexer.contains(n));
            indexer.remove(n);
            assert!(!indexer.contains(n));
        }

        assert_eq!(indexer.len(), 0);
        assert!(indexer.is_empty());
    }

    #[test]
    fn resize() {
        let mut indexer = Indexer::new();
        indexer.insert(0);
        assert!(indexer.contains(0));
        indexer.insert(2);
        assert!(indexer.contains(2));

        let index = indexer.capacity() * 2;
        indexer.insert(index);
        assert!(indexer.contains(index));
        assert!(indexer.contains(0));
        assert!(indexer.contains(2));
    }
}
