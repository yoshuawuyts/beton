pub(crate) use into_occupied::IntoOccupied;
pub(crate) use occupied::Occupied;
pub(crate) use unoccupied::UnOccupied;

mod into_occupied;
mod occupied;
mod unoccupied;

/// An indexing structure implemented as a bit-tree.
#[derive(Debug)]
pub(crate) struct BitVec {
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
        Self {
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
const fn compute_index(index: usize) -> (usize, usize) {
    let byte_position = index / (usize::BITS as usize);
    let bit_mask = 1 << (index % usize::BITS as usize);
    (byte_position, bit_mask)
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn index() {
        assert_eq!(compute_index(0), (0, 0b00001));
        assert_eq!(compute_index(1), (0, 0b00010));
        assert_eq!(compute_index(2), (0, 0b00100));

        assert_eq!(compute_index(64 - 1), (0, 1 << 63));
        assert_eq!(compute_index(64 + 0), (1, 0b00001));
        assert_eq!(compute_index(64 + 1), (1, 0b00010));
        assert_eq!(compute_index(64 + 2), (1, 0b00100));

        assert_eq!(compute_index(128 - 1), (1, 1 << 63));
        assert_eq!(compute_index(128 + 0), (2, 0b00001));
        assert_eq!(compute_index(128 + 1), (2, 0b00010));
        assert_eq!(compute_index(128 + 2), (2, 0b00100));
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
