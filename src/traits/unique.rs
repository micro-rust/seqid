//! Common trait for an non-infinite Unique ID generator.
//! This generator may exhaust it's output space and return `None`.
//! The implementors of this trait **MUST** guarantee that the generated values
//! are not repeated.
//! This trait does **NOT** guarantee an infinite amount of elements.



pub trait UniqueGenerator {
    /// The output of this Unique ID generator.
    type Output;

    /// Requests the next ID.
    fn next(&mut self) -> Option<Self::Output>;
}
