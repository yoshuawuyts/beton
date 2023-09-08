use crate::bit_tree::BitTree;
use crate::Key;

use std::mem::{self, MaybeUninit};

/// A slab allocator
#[derive(Debug)]
pub struct Slab<T> {
    index: BitTree,
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
            index: BitTree::with_capacity(capacity),
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
        if self.index.is_full() {
            self.reserve(1);
        }

        let index = self.index.next_unoccupied().unwrap();
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
