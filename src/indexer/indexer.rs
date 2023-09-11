use super::bit_array::{self, BitArray};
use super::bool_vec::{self, BoolVec};

/// How many bits should our in-line strucutre hold?
const CAPACITY: usize = 2;

#[derive(Debug)]
enum Inner {
    BoolVec(BoolVec),
    BitVec(BitArray<CAPACITY>),
}

/// An indexing structure with variable backends.
#[derive(Debug)]
pub(crate) struct Indexer {
    inner: Inner,
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new()
    }
}

impl Indexer {
    /// Create an empty instance of the `index`
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            inner: Inner::BitVec(BitArray::new()),
        }
    }

    /// Initialize the index with capacity
    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        if capacity < (u64::BITS as usize * CAPACITY) {
            Self::new()
        } else {
            Self {
                inner: Inner::BoolVec(BoolVec::with_capacity(capacity)),
            }
        }
    }

    /// Insert an entry into the index
    #[inline]
    pub(crate) fn insert(&mut self, index: usize) {
        match self.inner {
            Inner::BoolVec(ref mut vec) => vec.insert(index),
            Inner::BitVec(ref mut vec) => {
                // Bitvec has a fixed capacity. If we're going to write out of
                // bounds we should switch over to a `BoolVec` instead.
                let capacity = vec.capacity();
                if (index + 1) == capacity {
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
            Inner::BoolVec(ref mut vec) => vec.remove(index),
            Inner::BitVec(ref mut vec) => vec.remove(index),
        }
    }

    /// Clear the entire index
    #[inline]
    pub(crate) fn clear(&mut self) {
        match self.inner {
            Inner::BoolVec(ref mut vec) => vec.clear(),
            Inner::BitVec(ref mut vec) => vec.clear(),
        }
    }

    /// Returns `true` if the index contains a value
    #[inline]
    pub(crate) fn contains(&self, index: usize) -> bool {
        match self.inner {
            Inner::BoolVec(ref vec) => vec.contains(index),
            Inner::BitVec(ref vec) => vec.contains(index),
        }
    }

    /// How many items are currently contained?
    #[inline]
    pub(crate) fn len(&self) -> usize {
        match self.inner {
            Inner::BoolVec(ref vec) => vec.len(),
            Inner::BitVec(ref vec) => vec.len(),
        }
    }

    /// Is the structure empty?
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        match self.inner {
            Inner::BoolVec(ref vec) => vec.is_empty(),
            Inner::BitVec(ref vec) => vec.is_empty(),
        }
    }

    /// What is the current capacity?
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        match &self.inner {
            Inner::BoolVec(vec) => vec.capacity(),
            Inner::BitVec(vec) => vec.capacity(),
        }
    }

    /// Resize the Index
    pub(crate) fn resize(&mut self, new_len: usize) {
        match &mut self.inner {
            Inner::BoolVec(vec) => vec.resize(new_len),
            Inner::BitVec(vec) => {
                if new_len > vec.capacity() {
                    let mut bool_vec = BoolVec::with_capacity(new_len);
                    for index in vec.occupied() {
                        bool_vec.insert(index);
                    }
                    self.inner = Inner::BoolVec(bool_vec);
                }
            }
        }
    }

    /// Create an iterator over the indexes occupied by items.
    pub(crate) fn occupied(&self) -> Occupied {
        Occupied::new(self)
    }

    /// Create an iterator over the indexes occupied by items.
    pub(crate) fn into_occupied(self) -> IntoOccupied {
        IntoOccupied::new(self)
    }

    /// Create an iterator over the indexes not occupied by items.
    pub(crate) fn unoccupied(&self) -> UnOccupied {
        UnOccupied::new(self)
    }
}

#[derive(Debug)]
enum OccupiedInner<'a> {
    BoolVec(bool_vec::Occupied<'a>),
    BitVec(bit_array::Occupied<'a, CAPACITY>),
}

#[derive(Debug)]
pub(crate) struct Occupied<'a>(OccupiedInner<'a>);

impl<'a> Occupied<'a> {
    #[inline]
    fn new(bit_tree: &'a Indexer) -> Self {
        match bit_tree.inner {
            Inner::BoolVec(ref vec) => {
                let occupied = vec.occupied();
                Self(OccupiedInner::BoolVec(occupied))
            }
            Inner::BitVec(ref vec) => {
                let occupied = vec.occupied();
                Self(OccupiedInner::BitVec(occupied))
            }
        }
    }
}

impl<'a> Iterator for Occupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OccupiedInner::BoolVec(ref mut vec) => vec.next(),
            OccupiedInner::BitVec(ref mut vec) => vec.next(),
        }
    }
}

#[derive(Debug)]
enum UnOccupiedInner<'a> {
    BoolVec(bool_vec::UnOccupied<'a>),
    BitVec(bit_array::UnOccupied<'a, CAPACITY>),
}

#[derive(Debug)]
pub(crate) struct UnOccupied<'a>(UnOccupiedInner<'a>);

impl<'a> UnOccupied<'a> {
    #[inline]
    fn new(bit_tree: &'a Indexer) -> Self {
        match bit_tree.inner {
            Inner::BoolVec(ref vec) => {
                let unoccupied = vec.unoccupied();
                Self(UnOccupiedInner::BoolVec(unoccupied))
            }
            Inner::BitVec(ref vec) => {
                let unoccupied = vec.unoccupied();
                Self(UnOccupiedInner::BitVec(unoccupied))
            }
        }
    }
}

impl<'a> Iterator for UnOccupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            UnOccupiedInner::BoolVec(ref mut vec) => vec.next(),
            UnOccupiedInner::BitVec(ref mut vec) => match vec.next() {
                Some(index) => Some(index),
                None => Some(u64::BITS as usize * CAPACITY),
            },
        }
    }
}

#[derive(Debug)]
enum IntoOccupiedInner {
    BoolVec(bool_vec::IntoOccupied),
    BitVec(bit_array::IntoOccupied<CAPACITY>),
}

#[derive(Debug)]
pub(crate) struct IntoOccupied(IntoOccupiedInner);

impl IntoOccupied {
    #[inline]
    fn new(bit_tree: Indexer) -> Self {
        match bit_tree.inner {
            Inner::BoolVec(vec) => {
                let occupied = vec.into_occupied();
                Self(IntoOccupiedInner::BoolVec(occupied))
            }
            Inner::BitVec(vec) => {
                let occupied = vec.into_occupied();
                Self(IntoOccupiedInner::BitVec(occupied))
            }
        }
    }
}

impl Iterator for IntoOccupied {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IntoOccupiedInner::BoolVec(ref mut vec) => vec.next(),
            IntoOccupiedInner::BitVec(ref mut vec) => vec.next(),
        }
    }
}
