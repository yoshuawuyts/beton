use std::mem;

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
    #[inline]
    pub(crate) fn insert(&mut self, index: usize) {
        self.entries[index] = true;
        self.count += 1;
    }

    /// Remove an entry from the index
    #[inline]
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
    #[inline]
    pub(crate) fn contains(&self, index: usize) -> bool {
        match self.entries.get(index) {
            Some(value) => *value,
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
        self.count == 0
    }

    /// What is the current capacity?
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.entries.len()
    }

    /// Resize the Index
    #[inline]
    pub(crate) fn resize(&mut self, new_len: usize) {
        let current_length = self.entries.len();
        self.entries.resize(new_len, false);

        if new_len < current_length {
            self.count = self.occupied().count();
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
pub(crate) struct Occupied<'a> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items are we yet to see?
    remaining: usize,
    /// The bit tree containing the data
    bool_vec: &'a BoolVec,
}

impl<'a> Occupied<'a> {
    #[inline]
    fn new(bool_vec: &'a BoolVec) -> Self {
        Self {
            cursor: 0,
            remaining: bool_vec.len(),
            bool_vec,
        }
    }
}

impl<'a> Iterator for Occupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bool_vec.entries.len() {
            self.cursor += 1;
            match self.bool_vec.contains(index) {
                true => {
                    self.remaining -= 1;
                    return Some(index);
                }
                false => continue,
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct UnOccupied<'a> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items remain?
    remaining: usize,
    /// The bit tree containing the data
    bool_vec: &'a BoolVec,
}

impl<'a> UnOccupied<'a> {
    #[inline]
    fn new(bool_vec: &'a BoolVec) -> Self {
        Self {
            cursor: 0,
            remaining: bool_vec.capacity() - bool_vec.len(),
            bool_vec,
        }
    }
}

impl<'a> Iterator for UnOccupied<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bool_vec.entries.len() {
            self.cursor += 1;
            match self.bool_vec.contains(index) {
                false => {
                    self.remaining -= 1;
                    return Some(index);
                }
                true => continue,
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct IntoOccupied {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items remain?
    remaining: usize,
    /// The bit tree containing the data
    bool_vec: BoolVec,
}

impl IntoOccupied {
    #[inline]
    fn new(bool_vec: BoolVec) -> Self {
        Self {
            cursor: 0,
            remaining: bool_vec.len(),
            bool_vec,
        }
    }
}

impl Iterator for IntoOccupied {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bool_vec.entries.len() {
            self.cursor += 1;
            match self.bool_vec.contains(index) {
                true => {
                    self.remaining -= 1;
                    return Some(index);
                }
                false => continue,
            }
        }
        None
    }
}
