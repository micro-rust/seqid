//! Implementations of the sequential ID generator traits.


#[cfg(feature = "alloc")]
mod hashmap;

mod unsigned;


#[cfg(feature = "alloc")]
pub use hashmap::SeqHashMap;

pub use unsigned::*;
