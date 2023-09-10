use super::bool_vec::{self, BoolVec};

#[derive(Debug)]
enum Inner {
    BoolVec(BoolVec),
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
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            inner: Inner::BoolVec(BoolVec::new()),
        }
    }

    /// Initialize the index with capacity
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Inner::BoolVec(BoolVec::with_capacity(capacity)),
        }
    }

    /// Insert an entry into the index
    pub(crate) fn insert(&mut self, index: usize) {
        match self.inner {
            Inner::BoolVec(ref mut bool_vec) => bool_vec.insert(index),
        }
    }

    /// Remove an entry from the index
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        match self.inner {
            Inner::BoolVec(ref mut bool_vec) => bool_vec.remove(index),
        }
    }

    /// Clear the entire index
    pub(crate) fn clear(&mut self) {
        match self.inner {
            Inner::BoolVec(ref mut bool_vec) => bool_vec.clear(),
        }
    }

    /// Returns `true` if the index contains a value
    pub(crate) fn contains(&self, index: usize) -> bool {
        match self.inner {
            Inner::BoolVec(ref bool_vec) => bool_vec.contains(index),
        }
    }

    /// Find the index of the next unoccupied item
    ///
    /// Returns `None` if there are no free indexes available
    pub(crate) fn next_unoccupied(&self) -> Option<usize> {
        match self.inner {
            Inner::BoolVec(ref bool_vec) => bool_vec.next_unoccupied(),
        }
    }

    /// How many items are currently contained?
    pub(crate) fn len(&self) -> usize {
        match self.inner {
            Inner::BoolVec(ref bool_vec) => bool_vec.len(),
        }
    }

    /// Is the structure empty?
    pub(crate) fn is_empty(&self) -> bool {
        match self.inner {
            Inner::BoolVec(ref bool_vec) => bool_vec.is_empty(),
        }
    }

    /// Can we add more items?
    pub(crate) fn is_full(&self) -> bool {
        match &self.inner {
            Inner::BoolVec(bool_vec) => bool_vec.is_full(),
        }
    }

    /// What is the current capacity?
    pub(crate) fn capacity(&self) -> usize {
        match &self.inner {
            Inner::BoolVec(bool_vec) => bool_vec.capacity(),
        }
    }

    /// Resize the Index
    pub(crate) fn resize(&mut self, new_len: usize) {
        match &mut self.inner {
            Inner::BoolVec(bool_vec) => bool_vec.resize(new_len),
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

    // /// Create an iterator over the indexes not occupied by items.
    // pub(crate) fn unoccupied(&self) -> impl Iterator<Item = usize> + '_ {
    //     match &self.inner {
    //         Inner::BoolVec(bool_vec) => bool_vec.unoccupied(),
    //     }
    // }
}

#[derive(Debug)]
enum OccupiedInner<'a> {
    BoolVec(bool_vec::Occupied<'a>),
}

#[derive(Debug)]
pub(crate) struct Occupied<'a>(OccupiedInner<'a>);

impl<'a> Occupied<'a> {
    fn new(bit_tree: &'a Indexer) -> Self {
        match bit_tree.inner {
            Inner::BoolVec(ref bool_vec) => {
                let occupied = bool_vec.occupied();
                Self(OccupiedInner::BoolVec(occupied))
            }
        }
    }
}

impl<'a> Iterator for Occupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            OccupiedInner::BoolVec(ref mut bv) => bv.next(),
        }
    }
}

#[derive(Debug)]
enum IntoOccupiedInner {
    BoolVec(bool_vec::IntoOccupied),
}

#[derive(Debug)]
pub(crate) struct IntoOccupied(IntoOccupiedInner);

impl IntoOccupied {
    fn new(bit_tree: Indexer) -> Self {
        match bit_tree.inner {
            Inner::BoolVec(bool_vec) => {
                let occupied = bool_vec.into_occupied();
                Self(IntoOccupiedInner::BoolVec(occupied))
            }
        }
    }
}

impl Iterator for IntoOccupied {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IntoOccupiedInner::BoolVec(ref mut bv) => bv.next(),
        }
    }
}
