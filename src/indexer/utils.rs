/// Compute the index and bitmask for a given index. This is used to
/// first index into a slice, and then produces a bitmask to access the right
/// bit of that slice.
#[inline]
pub(crate) const fn compute_index(index: usize) -> (usize, usize) {
    let byte_position = index / (usize::BITS as usize);
    let bit_mask = 1 << (index % usize::BITS as usize);
    (byte_position, bit_mask)
}

#[cfg(test)]
mod test {
    use super::*;
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
}
