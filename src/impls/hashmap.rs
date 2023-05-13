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
        hash_map::{
            Keys, Values
        },
    },
};



/// Internal type to distinguish `SeqHashMap` instances.
type SeqHashMapUid = usize;



pub struct SeqHashMap<I: UniqueGenerator, T> {
    /// The UID of this `SeqHashMap`. Used to distinguish between instances of
    /// a type. This avoids using the reservation of an instance with a
    /// different instance.
    uid: SeqHashMapUid,

    /// Sequential ID generator.
    gen: I,

    /// Internal `HashMap`.
    map: HashMap<<I as UniqueGenerator>::Output, T>,
}

impl<I: UniqueGenerator, T> SeqHashMap<I, T> {
    /// Attempts to create a new `SeqHashMap`.
    /// UNSAFETY : This is currently single threaded.
    pub fn new() -> Option<Self> where I : Default {
        use std::sync::Mutex;

        // Global UID generator for `SeqHashMap`.
        static UID: Mutex<SeqHashMapUid> = Mutex::new( 0 );

        Some( Self { uid: UID.lock().expect("Poisoned global lock for UID").next()?, gen: Default::default(), map: HashMap::new(), } )
    }

    /// Returns the capacity of the internal map without needing to reallocate.
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Returns the number of elements in the internal map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns the value that matches the given key. If no value matches the key
    /// returns `None`. This method does not perform any kind of origin validation
    /// on the key (the key could be generated by a different map).
    pub fn get(&self, key: &<I as UniqueGenerator>::Output) -> Option<&T> where <I as UniqueGenerator>::Output : Eq + Hash {
        self.map.get(key)
    }

    /// Inserts a new item into the `HashMap` and returns the key to it.
    /// This method returns `None` if the UID generator cannot produce more values.
    /// This method panics if the UID generator produces a repeated value.
    pub fn insert(&mut self, t: T) -> Option<<I as UniqueGenerator>::Output> where <I as UniqueGenerator>::Output : Clone + Eq + Hash {
        // Get the next UID.
        let uid = self.gen.next()?;

        // Insert the node into the map.
        match self.map.insert(uid.clone(), t) {
            Some(_) => panic!("Unique ID must not overwrite previous elements"),
            _ => Some(uid),
        }
    }

    /// Iterates over the internal hashmap's data.
    pub fn iter(&self) -> impl Iterator<Item=(&<I as UniqueGenerator>::Output, &T)> {
        self.map.iter()
    }

    /// Returns the keys inserted in the HashMap.
    pub fn keys(&self) -> Keys<<I as UniqueGenerator>::Output, T> {
        self.map.keys()
    }

    /// Returns the values inserted in the HashMap.
    pub fn values(&self) -> Values<<I as UniqueGenerator>::Output, T> {
        self.map.values()
    }

    /// Reserves an ID in the `HashMap`.
    pub fn reserve(&mut self) -> Option<Reservation<<I as UniqueGenerator>::Output>> where <I as UniqueGenerator>::Output : Clone + Eq + Hash{
        // Get the next UID.
        let uid = self.gen.next()?;

        // Create the reservation.
        Some( Reservation { uid, origin: self.uid, } )
    }

    /// Redeems a reservation and inserts the given item into the `HashMap`.
    /// This method fails if the reservation did not originate from this map.
    pub fn redeem(&mut self, reservation: Reservation<<I as UniqueGenerator>::Output>, t: T) -> Result<<I as UniqueGenerator>::Output, Reservation<<I as UniqueGenerator>::Output>> where <I as UniqueGenerator>::Output : Clone + Eq + Hash {
        // Check that the origin of the reservation is this instance.
        if reservation.origin != self.uid {
            return Err( reservation );
        }

        // Get the UID.
        let uid = reservation.uid;

        // Insert the node into the map.
        match self.map.insert(uid.clone(), t) {
            Some(_) => panic!("Unique ID must not overwrite previous elements"),
            _ => Ok(uid),
        }
    }
}



/// A reserved ID to insert into a `SeqHashMap`.
pub struct Reservation<R> {
    /// UID of the reservation.
    uid: R,

    /// ID of the generator struct.
    origin: SeqHashMapUid,
}
