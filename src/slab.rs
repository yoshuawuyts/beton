use crate::indexer::Indexer;
use crate::{IntoIter, IntoValues, Iter, IterMut, Key, Keys, Values, ValuesMut};

use std::mem::{self, MaybeUninit};
use std::ops::{Index, IndexMut};

/// A slab allocator
#[derive(Default)]
pub struct Slab<T> {
    pub(crate) index: Indexer,
    pub(crate) entries: Vec<MaybeUninit<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Slab<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slab").field("index", &self.index).finish()
    }
}

impl<T> Slab<T> {
    /// Creates an empty `Slab`.
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty `Slab` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            index: Indexer::with_capacity(capacity),
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.index.clear();
        self.entries.clear();
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, key: Key) -> bool {
        self.index.contains(key.into())
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: Key) -> Option<&T> {
        if self.contains_key(key) {
            self.entries.get(usize::from(key)).map(|v| {
                // SAFETY: We just validated that the index contains a key
                // for this value, meaning we can safely assume that this
                // value is initialized.
                unsafe { v.assume_init_ref() }
            })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        if self.contains_key(key) {
            self.entries.get_mut(usize::from(key)).map(|v| {
                // SAFETY: We just validated that the index contains a key
                // for this value, meaning we can safely assume that this
                // value is initialized.
                unsafe { v.assume_init_mut() }
            })
        } else {
            None
        }
    }

    /// Inserts a value into the slab
    ///
    /// Returns the key for the entry.
    pub fn insert(&mut self, value: T) -> Key {
        let index = self.index.unoccupied().next().unwrap();
        self.index.insert(index);
        self.entries.insert(index, MaybeUninit::new(value));
        Key::new(index)
    }

    /// Reserves capacity for at least additional more elements to be inserted.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds isize::MAX bytes.
    pub fn reserve(&mut self, additional: usize) {
        let new_len = self.index.capacity() + additional;
        self.resize(new_len);
    }

    /// Resizes the `Slab` in-place so that `len` is equal to `new_len`.
    pub fn resize(&mut self, new_len: usize) {
        self.index.resize(new_len);
        self.entries.resize_with(new_len, || MaybeUninit::uninit());
    }

    /// Remove and return the value associated with the given key.
    ///
    /// The key is then released and may be associated with future stored values.
    pub fn remove(&mut self, key: Key) -> Option<T> {
        let index = key.into();
        if self.index.remove(index) {
            let mut output = MaybeUninit::uninit();
            mem::swap(&mut self.entries[index], &mut output);
            // SAFETY: we just confirmed that there was in fact an entry at this index
            Some(unsafe { output.assume_init() })
        } else {
            None
        }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.index.capacity()
    }

    /// Returns an iterator over all key-value pairs.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }

    /// Returns an iterator over key-value pairs that allows modifying each
    /// value.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.into_iter()
    }

    /// Returns an iterator over all keys.
    ///
    /// The iterator yields all keys from start to end.
    pub fn keys(&self) -> Keys<'_> {
        Keys::new(self)
    }

    /// Returns an iterator over all values.
    ///
    /// The iterator yields all values from start to end.
    pub fn values(&self) -> Values<'_, T> {
        Values::new(self)
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all values from start to end.
    pub fn values_mut(&mut self) -> ValuesMut<'_, T> {
        ValuesMut::new(self)
    }

    /// Consumes `self` and returns an iterator over all values.
    ///
    /// The iterator yields all values from start to end.
    pub fn into_values(self) -> IntoValues<T> {
        IntoValues::new(self)
    }
}

impl<T> IntoIterator for Slab<T> {
    type Item = (Key, T);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a Slab<T> {
    type Item = (Key, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a mut Slab<T> {
    type Item = (Key, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

impl<T> FromIterator<T> for Slab<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let capacity = iter.size_hint().1.unwrap_or(0);
        let mut slab = Slab::with_capacity(capacity);
        for value in iter {
            slab.insert(value);
        }
        slab
    }
}

impl<T> Extend<T> for Slab<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let additional = iter.size_hint().1.unwrap_or(0);
        self.reserve(additional);
        for value in iter {
            self.insert(value);
        }
    }
}

/// Returns a reference to the value corresponding to the supplied key.
///
/// # Panics
///
/// Panics if the key is not present in the `Slab`.
impl<T> Index<Key> for Slab<T> {
    type Output = T;

    fn index(&self, index: Key) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/// Returns a mutable reference to the value corresponding to the supplied key.
///
/// # Panics
///
/// Panics if the key is not present in the `Slab`.
impl<T> IndexMut<Key> for Slab<T> {
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> Drop for Slab<T> {
    fn drop(&mut self) {
        for index in self.index.occupied() {
            // SAFETY: we're going over all items marked as "occupied" and
            // dropping them in-place.
            unsafe { self.entries[index].assume_init_drop() }
        }
    }
}
