//! Common trait for an non-infinite ID generator.
//! This generator may exhaust it's output space and return `None`.



pub trait FiniteSequential {
    /// The output of this sequential ID generator.
    type Output;

    /// Requests the next ID.
    fn next(&mut self) -> Option<Self::Output>;
}
