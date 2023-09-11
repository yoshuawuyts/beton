use std::collections;

/// An indexing structure implemented as a bit-tree.
#[derive(Debug, Default)]
pub(crate) struct BTreeSet {
    entries: collections::BTreeSet<usize>,
}

impl BTreeSet {
    /// Create an empty instance of the `index`
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            entries: collections::BTreeSet::new(),
        }
    }

    /// Insert an entry into the index
    #[inline]
    pub(crate) fn insert(&mut self, index: usize) {
        self.entries.insert(index);
    }

    /// Remove an entry from the index
    #[inline]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        self.entries.remove(&index)
    }

    /// Clear the entire index
    pub(crate) fn clear(&mut self) {
        self.entries.clear()
    }

    /// Returns `true` if the index contains a value
    #[inline]
    pub(crate) fn contains(&self, index: usize) -> bool {
        self.entries.contains(&index)
    }

    /// How many items are currently contained?
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    /// Is the structure empty?
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
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
pub(crate) struct Occupied<'a>(collections::btree_set::Iter<'a, usize>);

impl<'a> Occupied<'a> {
    #[inline]
    fn new(set: &'a BTreeSet) -> Self {
        Self(set.entries.iter())
    }
}

impl<'a> Iterator for Occupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }
}

#[derive(Debug)]
pub(crate) struct UnOccupied<'a> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items remain?
    max: usize,
    /// The bit tree containing the data
    set: &'a BTreeSet,
}

impl<'a> UnOccupied<'a> {
    #[inline]
    fn new(set: &'a BTreeSet) -> Self {
        Self {
            cursor: 0,
            max: set.entries.last().copied().unwrap_or(0),
            set,
        }
    }
}

impl<'a> Iterator for UnOccupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.max {
            return Some(self.cursor);
        }

        for index in self.cursor..self.max {
            self.cursor += 1;
            match self.set.contains(index) {
                false => return Some(index),
                true => continue,
            }
        }
        Some(self.cursor)
    }
}

#[derive(Debug)]
pub(crate) struct IntoOccupied(collections::btree_set::IntoIter<usize>);

impl IntoOccupied {
    #[inline]
    fn new(set: BTreeSet) -> Self {
        Self(set.entries.into_iter())
    }
}

impl Iterator for IntoOccupied {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
