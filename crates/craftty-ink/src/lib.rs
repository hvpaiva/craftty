//! A declarative terminal styling library.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod color;
pub mod error;

/// Returns the crate version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_valid() {
        assert!(!version().is_empty());
    }
}
