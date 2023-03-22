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



/// Internal type to distinguish `SeqHashMap` instances.
type SeqHashMapUid = usize;



pub struct SeqHashMap<I: UniqueGenerator, T> {
    /// The UID of this `SeqHashMap`. Used to distinhuish between instances of
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
    pub fn new() -> Option<Self> where I : Default {
        use std::sync::Mutex;

        // Global UID generator for `SeqHashMap`.
        static UID: Mutex<SeqHashMapUid> = Mutex::new( 0 );

        Some( Self { uid: UID.lock().expect("Poisoned global lock for UID").next()?, gen: Default::default(), map: HashMap::new(), } )
    }

    /// Inserts a new item into the `HashMap` and returns the key to it.
    /// This method returns `None` if the UID generator cannot produce mmore values.
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

    /// Reserves an ID in the `HashMap`.
    pub fn reserve(&mut self) -> Option<Reservation<<I as UniqueGenerator>::Output>> where <I as UniqueGenerator>::Output : Clone + Eq + Hash{
        // Get the next UID.
        let uid = self.gen.next()?;

        // Create the reservation.
        Some( Reservation { uid, origin: self.uid, } )
    }

    /// Redeems a reservation and inserts the given item into the `HashMap`.
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
