//! A tree-backed slab allocator
//!
//! # Differences with the slab crate
//!
//! Entries into the `slab` crate's slab structure are backed by a linked lists,
//! which makes it expensive to iterate over. According to [slab's
//! iterator docs](https://docs.rs/slab/0.4.9/slab/struct.Slab.html#method.iter):
//!
//! > \[`Slab::iter`\] should generally be avoided as it is not efficient.
//! > Iterators must iterate over every slot in the slab even if it is vacant. As
//! > such, a slab with a capacity of 1 million but only one stored value must
//! > still iterate the million slots.
//!
//! This crate uses a tree to hold the indexes instead, ensuring that iterating
//! over the entries in the slab remains cheap.
//!
//! # Examples
//!
//! ```text
//! // tbi
//! ```

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

mod bit_tree;
mod iter;
mod key;
mod slab;

pub use self::slab::Slab;
pub use iter::{IntoIter, Iter};
pub use key::Key;
