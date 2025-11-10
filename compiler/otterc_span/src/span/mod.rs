//! definition for the `Span` type.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: u32,
    end: u32,
}
