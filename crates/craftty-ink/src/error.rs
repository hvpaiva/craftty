//! Error types for craftty-ink.

use std::fmt;

/// An error returned when parsing an invalid color string.
///
/// # Examples
///
/// ```
/// use craftty_ink::color::Color;
/// use craftty_ink::error::ParseColorError;
///
/// let result: Result<Color, ParseColorError> = "banana".parse();
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError {
    input: String,
}

impl ParseColorError {
    pub(crate) fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }

    /// Returns the input string that failed to parse.
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid color: {:?}", self.input)
    }
}

impl std::error::Error for ParseColorError {}
