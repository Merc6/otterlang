//! definition for the `Span` type.

/// A byte range, typically used for representing a slice in source-text
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// the ending position of the range
    end: u32,
    /// the starting position of the range
    start: u32,
}
