use std::{mem, ops::Not};

/// An indexing structure implemented as a bit-tree.
#[derive(Debug, Default)]
pub(crate) struct BoolVec {
    entries: Vec<bool>,
    count: usize,
}

impl BoolVec {
    /// Create an empty instance of the `index`
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Initialize the index with capacity
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: vec![false; capacity],
            count: 0,
        }
    }

    /// Insert an entry into the index
    pub(crate) fn insert(&mut self, index: usize) {
        self.entries[index] = true;
        self.count += 1;
    }

    /// Remove an entry from the index
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        let mut ret = false;
        let entry = match self.entries.get_mut(index) {
            Some(entry) => entry,
            None => return false,
        };
        mem::swap(entry, &mut ret);
        if ret {
            self.count -= 1;
        }
        ret
    }

    /// Clear the entire index
    pub(crate) fn clear(&mut self) {
        self.entries.fill(false);
        self.count = 0;
    }

    /// Returns `true` if the index contains a value
    pub(crate) fn contains(&self, index: usize) -> bool {
        match self.entries.get(index) {
            Some(value) => *value,
            None => false,
        }
    }

    /// Find the index of the next unoccupied item
    ///
    /// Returns `None` if there are no free indexes available
    pub(crate) fn next_unoccupied(&self) -> Option<usize> {
        self.unoccupied().next()
    }

    /// How many items are currently contained?
    pub(crate) fn len(&self) -> usize {
        self.count
    }

    /// Is the structure empty?
    pub(crate) fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Can we add more items?
    pub(crate) fn is_full(&self) -> bool {
        self.count == self.entries.len()
    }

    /// What is the current capacity?
    pub(crate) fn capacity(&self) -> usize {
        self.entries.len()
    }

    /// Resize the Index
    pub(crate) fn resize(&mut self, new_len: usize) {
        let current_length = self.entries.len();
        self.entries.resize(new_len, false);

        if new_len < current_length {
            self.count = self.occupied().count();
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
    pub(crate) fn unoccupied(&self) -> impl Iterator<Item = usize> + '_ {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(index, occupied)| occupied.not().then_some(index))
    }
}

#[derive(Debug)]
pub(crate) struct Occupied<'a> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items have we seen?
    seen: usize,
    /// Have we finished?
    is_done: bool,
    /// The bit tree containing the data
    bit_tree: &'a BoolVec,
}

impl<'a> Occupied<'a> {
    fn new(bit_tree: &'a BoolVec) -> Self {
        Self {
            cursor: 0,
            is_done: false,
            seen: 0,
            bit_tree,
        }
    }
}

impl<'a> Iterator for Occupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        for index in self.cursor..self.bit_tree.entries.len() {
            self.cursor += 1;
            match self.bit_tree.entries[index] {
                true => {
                    self.seen += 1;
                    if self.seen == self.bit_tree.entries.len() {
                        self.is_done = true;
                    }
                    return Some(index);
                }
                false => continue,
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct IntoOccupied {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items have we seen?
    seen: usize,
    /// Have we finished?
    is_done: bool,
    /// The bit tree containing the data
    bit_tree: BoolVec,
}

impl IntoOccupied {
    fn new(bit_tree: BoolVec) -> Self {
        Self {
            cursor: 0,
            is_done: false,
            seen: 0,
            bit_tree,
        }
    }
}

impl Iterator for IntoOccupied {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        for index in self.cursor..self.bit_tree.entries.len() {
            self.cursor += 1;
            match self.bit_tree.entries[index] {
                true => {
                    self.seen += 1;
                    if self.seen == self.bit_tree.entries.len() {
                        self.is_done = true;
                    }
                    return Some(index);
                }
                false => continue,
            }
        }
        None
    }
}
