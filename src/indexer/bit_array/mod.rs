use super::utils::compute_index;
pub(crate) use into_occupied::IntoOccupied;
pub(crate) use occupied::Occupied;
pub(crate) use unoccupied::UnOccupied;

mod into_occupied;
mod occupied;
mod unoccupied;

/// An indexing structure implemented as a bit-tree.
#[derive(Debug)]
pub(crate) struct BitArray<const N: usize> {
    entries: [usize; N],
}

impl<const N: usize> Default for BitArray<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> BitArray<N> {
    /// Create an empty instance of the `index`
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self { entries: [0; N] }
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
    }

    /// Remove an entry from the index
    #[inline]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        let (index, mask) = compute_index(index);
        let ret = self.contains(index);
        match self.entries.get_mut(index) {
            Some(entry) => {
                *entry &= !mask;
                ret
            }
            None => false,
        }
    }

    /// Clear the entire index
    #[inline]
    pub(crate) fn clear(&mut self) {
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
        self.entries
            .iter()
            .copied()
            .map(|entry| entry.count_ones() as usize)
            .sum()
    }

    /// Is the structure empty?
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        for entry in self.entries {
            if entry != 0 {
                return false;
            }
        }
        true
    }

    /// What is the current capacity?
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        usize::BITS as usize * N
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        const LEN: usize = 2;
        let mut arr: BitArray<LEN> = BitArray::new();
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
        const LEN: usize = 2;
        let mut arr: BitArray<LEN> = BitArray::new();
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
