use super::BitArray;
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
    pub(crate) fn new(bit_array: &'a BitArray<N>) -> Self {
        Self {
            cursor: 0,
            remaining: bit_array.len(),
            bit_array,
        }
    }
}

impl<'a, const N: usize> Iterator for Occupied<'a, N> {
    type Item = usize;

    #[inline]
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
