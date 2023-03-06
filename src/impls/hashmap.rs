//! Hashmap implementation with a sequential key type.



use core::{
    hash::{
        Hash,
    },
};

use crate::{
    traits::{
        UniqueGenerator,
    },
};

use std::{
    collections::{
        HashMap,
    },
};


pub struct SeqHashMap<I: UniqueGenerator, T> {
    /// Sequential ID generator.
    uid: I,

    /// Internal `HashMap`.
    map: HashMap<<I as UniqueGenerator>::Output, T>,
}

impl<I: UniqueGenerator, T> SeqHashMap<I, T> {
    /// Inserts a new item into the `HashMap` and returns the key to it.
    /// This method returns `None` if the UID generator cannot produce mmore values.
    /// This method panics if the UID generator produces a repeated value.
    pub fn insert(&mut self, t: T) -> Option<<I as UniqueGenerator>::Output> where <I as UniqueGenerator>::Output : Clone + Eq + Hash {
        // Get the next UID.
        let uid = self.uid.next()?;

        // Insert the node into the map.
        match self.map.insert(uid.clone(), t) {
            Some(_) => panic!("Unique ID must not overwrite previous elements"),
            _ => Some(uid),
        }
    }
}

impl<I: UniqueGenerator + Default, T> Default for SeqHashMap<I, T> {
    fn default() -> Self {
        Self { uid: Default::default(), map: HashMap::new(), }
    }
}
