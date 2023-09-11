use bitvec::array::BitArray;

/// An indexing structure implemented as a bit-tree.
#[derive(Debug, Default)]
pub(crate) struct BitVec<const N: usize> {
    entries: BitArray<[usize; N]>,
    count: usize,
}

impl<const N: usize> BitVec<N> {
    /// Create an empty instance of the `index`
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self {
            entries: BitArray::ZERO,
            count: 0,
        }
    }

    /// Insert an entry into the index
    #[inline]
    pub(crate) fn insert(&mut self, index: usize) {
        let mut entry = self.entries.get_mut(index).unwrap();
        entry.set(true);
        self.count += 1;
    }

    /// Remove an entry from the index
    #[inline]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        match self.entries.get_mut(index).as_deref_mut() {
            Some(entry @ true) => {
                self.count -= 1;
                *entry = false;
                true
            }
            _ => false,
        }
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

    /// Create an iterator over the indexes occupied by items.
    #[inline]
    pub(crate) fn occupied(&self) -> Occupied<N> {
        Occupied::new(self)
    }

    /// Create an iterator over the indexes occupied by items.
    #[inline]
    pub(crate) fn into_occupied(self) -> IntoOccupied<N> {
        IntoOccupied::new(self)
    }

    /// Create an iterator over the indexes not occupied by items.
    #[inline]
    pub(crate) fn unoccupied(&self) -> UnOccupied<N> {
        UnOccupied::new(self)
    }
}

#[derive(Debug)]
pub(crate) struct Occupied<'a, const N: usize> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items are we yet to see?
    remaining: usize,
    /// The bit tree containing the data
    bool_vec: &'a BitVec<N>,
}

impl<'a, const N: usize> Occupied<'a, N> {
    #[inline]
    fn new(bool_vec: &'a BitVec<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bool_vec.len(),
            bool_vec,
        }
    }
}

impl<'a, const N: usize> Iterator for Occupied<'a, N> {
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
pub(crate) struct UnOccupied<'a, const N: usize> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items remain?
    remaining: usize,
    /// The bit tree containing the data
    bool_vec: &'a BitVec<N>,
}

impl<'a, const N: usize> UnOccupied<'a, N> {
    #[inline]
    fn new(bool_vec: &'a BitVec<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bool_vec.capacity() - bool_vec.len(),
            bool_vec,
        }
    }
}

impl<'a, const N: usize> Iterator for UnOccupied<'a, N> {
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
pub(crate) struct IntoOccupied<const N: usize> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items remain?
    remaining: usize,
    /// The bit tree containing the data
    bool_vec: BitVec<N>,
}

impl<const N: usize> IntoOccupied<N> {
    #[inline]
    fn new(bool_vec: BitVec<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bool_vec.len(),
            bool_vec,
        }
    }
}

impl<const N: usize> Iterator for IntoOccupied<N> {
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
