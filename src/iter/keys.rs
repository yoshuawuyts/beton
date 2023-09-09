use crate::bit_tree::Occupied;
use crate::{Key, Slab};

/// An borrowing iterator over items in the `Slab`.
#[derive(Debug)]
pub struct Keys<'a> {
    occupied: Occupied<'a>,
}

impl<'a> Keys<'a> {
    pub(crate) fn new<T>(slab: &'a Slab<T>) -> Self {
        let occupied = slab.index.occupied();
        Self { occupied }
    }
}

impl<'a> Iterator for Keys<'a> {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.occupied.next().map(|index| index.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        let mut slab = crate::Slab::new();
        slab.insert(1);
        let key = slab.insert(2);
        slab.insert(3);
        slab.remove(key);
        let mut iter = Keys::new(&slab);
        assert_eq!(iter.next(), Some(0.into()));
        assert_eq!(iter.next(), Some(2.into()));
        assert_eq!(iter.next(), None);
    }
}
