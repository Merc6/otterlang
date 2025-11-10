//! definition for the `Span` type.

/// A byte range, typically used for representing a slice in source-text
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub struct Span {
    /// the ending position of the range
    end: u32,
    /// the starting position of the range
    start: u32,
}

impl Span {
    /// Creates a new instance of [`Span`]
    #[must_use = "This function is a constructor"]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { end, start }
    }
}

impl Span {
    /// An unrepresentable [`Span`]
    ///
    /// If you're looking for a [`Span`] to represent *no* span, you should use
    /// the [`None`] variant of `Option<Span>`
    pub const DUMMY: Self = Self::new(0, 0);
}
