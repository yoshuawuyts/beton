use crate::Key;
use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::mem::MaybeUninit;

/// A slab allocator
#[derive(Debug)]
pub struct Slab<T> {
    keys: BTreeSet<Key>,
    entries: Vec<MaybeUninit<T>>,
}

impl<T> Slab<T> {
    /// Creates an empty `Slab`.
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty `Slab` with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: BTreeSet::new(),
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.keys.clear();
        self.entries.clear();
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key<K>(&self, key: &K) -> bool
    where
        K: Borrow<Key>,
    {
        self.keys.contains(key.borrow())
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<K>(&self, key: &K) -> Option<&T>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        if self.contains_key(key) {
            self.entries.get(usize::from(*key)).map(|v| {
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
    pub fn get_mut<K>(&mut self, key: &K) -> Option<&mut T>
    where
        K: Borrow<Key>,
    {
        let key = key.borrow();
        if self.contains_key(key) {
            self.entries.get_mut(usize::from(*key)).map(|v| {
                // SAFETY: We just validated that the index contains a key
                // for this value, meaning we can safely assume that this
                // value is initialized.
                unsafe { v.assume_init_mut() }
            })
        } else {
            None
        }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, value: T) -> Option<T> {
        todo!()
    }

    /// Remove and return the value associated with the given key.
    ///
    /// The key is then released and may be associated with future stored values.
    pub fn remove(&mut self, key: Key) -> T {
        todo!()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        todo!()
    }
}
