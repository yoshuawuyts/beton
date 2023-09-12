use super::BitVec;

#[derive(Debug)]
pub(crate) struct UnOccupied<'a> {
    /// What is the current index of the cursor?
    cursor: usize,
    /// How many items remain?
    remaining: usize,
    /// The bit tree containing the data
    bit_array: &'a BitVec,
}

impl<'a> UnOccupied<'a> {
    #[inline]
    pub(crate) fn new(bit_array: &'a BitVec) -> Self {
        Self {
            cursor: 0,
            remaining: bit_array.capacity() - bit_array.len(),
            bit_array,
        }
    }
}

impl<'a> Iterator for UnOccupied<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        for index in self.cursor..self.bit_array.capacity() {
            // Check once per byte whether the entire byte is set. If it is we
            // can skip to the next byte. If it isn't, we iterate over it.
            if (index % usize::BITS as usize) == 0 {
                let byte_position = index / (usize::BITS as usize);
                if self.bit_array.entries[byte_position] == usize::MAX {
                    self.cursor += usize::BITS as usize;
                    continue;
                }
            } else {
                self.cursor += 1;
            }
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
