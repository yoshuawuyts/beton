/// An indexing structure implemented as a bit-tree.
#[derive(Debug)]
pub(crate) struct BitArray<const N: usize> {
    entries: [usize; N],
}

impl<const N: usize> Default for BitArray<N> {
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
        let (index, mask) = compute_index::<N>(index);
        self.entries[index] |= mask;
    }

    /// Remove an entry from the index
    #[inline]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        let (index, mask) = compute_index::<N>(index);
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
        let (index, mask) = compute_index::<N>(index);
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

fn compute_index<const N: usize>(index: usize) -> (usize, usize) {
    let byte_position = index / (usize::BITS as usize);
    let bit_mask = 1 << (index % usize::BITS as usize);
    (byte_position, bit_mask)
}

#[derive(Debug)]
pub(crate) struct Occupied<'a, const N: usize> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items are we yet to see?
    remaining: usize,
    /// The bit tree containing the data
    bit_array: &'a BitArray<N>,
}

impl<'a, const N: usize> Occupied<'a, N> {
    #[inline]
    fn new(bit_array: &'a BitArray<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bit_array.len(),
            bit_array,
        }
    }
}

impl<'a, const N: usize> Iterator for Occupied<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bit_array.capacity() {
            self.cursor += 1;
            match self.bit_array.contains(index) {
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
    bit_array: &'a BitArray<N>,
}

impl<'a, const N: usize> UnOccupied<'a, N> {
    #[inline]
    fn new(bit_array: &'a BitArray<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bit_array.capacity() - bit_array.len(),
            bit_array,
        }
    }
}

impl<'a, const N: usize> Iterator for UnOccupied<'a, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bit_array.capacity() {
            self.cursor += 1;
            match self.bit_array.contains(index) {
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
    bit_array: BitArray<N>,
}

impl<const N: usize> IntoOccupied<N> {
    #[inline]
    fn new(bit_array: BitArray<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bit_array.len(),
            bit_array,
        }
    }
}

impl<const N: usize> Iterator for IntoOccupied<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bit_array.capacity() {
            self.cursor += 1;
            match self.bit_array.contains(index) {
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
    fn index() {
        assert_eq!(compute_index::<2>(0), (0, 0b00001));
        assert_eq!(compute_index::<2>(1), (0, 0b00010));
        assert_eq!(compute_index::<2>(2), (0, 0b00100));

        assert_eq!(compute_index::<2>(64 - 1), (0, 1 << 63));
        assert_eq!(compute_index::<2>(64 + 0), (1, 0b00001));
        assert_eq!(compute_index::<2>(64 + 1), (1, 0b00010));
        assert_eq!(compute_index::<2>(64 + 2), (1, 0b00100));

        assert_eq!(compute_index::<2>(128 - 1), (1, 1 << 63));
        assert_eq!(compute_index::<2>(128 + 0), (2, 0b00001));
        assert_eq!(compute_index::<2>(128 + 1), (2, 0b00010));
        assert_eq!(compute_index::<2>(128 + 2), (2, 0b00100));
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
