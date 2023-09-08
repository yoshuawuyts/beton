/// An key into the [`Slab`](crate::Slab) structure.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Key(usize);

impl Key {
    pub(crate) fn new(index: usize) -> Key {
        Self(index)
    }
}

impl From<Key> for usize {
    #[inline(always)]
    fn from(value: Key) -> Self {
        value.0
    }
}

impl From<usize> for Key {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self(value)
    }
}
