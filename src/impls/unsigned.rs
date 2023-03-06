//! Unsigned integer sequential generators.



use core::sync::atomic::*;

use crate::traits::{
    FiniteSequential, UniqueGenerator,
};



macro_rules! raw {
    ([$($integer:ty),+]) => {
        $(
            impl FiniteSequential for $integer {
                type Output = Self;

                fn next(&mut self) -> Option<Self::Output> {
                    // End the sequence if the maximum number is reached.
                    if *self == Self::MAX {
                        return None;
                    }

                    // Get the next sequential ID.
                    let next = *self;

                    // Increment the internal value.
                    *self += 1;

                    Some(next)
                }
            }
        )+
    };
}



macro_rules! atomic {
    ([$($integer:ty => $out:ty),+]) => {        
        $(
            impl FiniteSequential for $integer {
                type Output = $out;

                fn next(&mut self) -> Option<Self::Output> {
                    // Get a mutable reference to the value.
                    let val = self.get_mut();

                    // End the sequence if the maximum number is reached.
                    if *val == <$out>::MAX {
                        return None;
                    }

                    // Get the next sequential ID.
                    let next = *val;

                    // Increment the internal value.
                    *val += 1;

                    Some(next)
                }
            }
        )+
    };
}

macro_rules! unique {
    ([$($finite:ty),+]) => {
        $(
            impl UniqueGenerator for $finite {
                type Output = <Self as FiniteSequential>::Output;

                #[inline(always)]
                fn next(&mut self) -> Option<Self::Output> {
                    <Self as FiniteSequential>::next(self)
                }
            }
        )+
    };
}



raw!([u8, u16, u32, u64, u128, usize]);
atomic!([AtomicU8 => u8, AtomicU16 => u16, AtomicU32 => u32, AtomicU64 => u64, AtomicUsize => usize]);
unique!([u8, AtomicU8, u16, AtomicU16, u32, AtomicU32, u64, AtomicU64, usize, AtomicUsize, u128]);
