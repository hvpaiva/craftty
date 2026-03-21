//! A declarative terminal styling library for Rust.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Placeholder to be replaced with actual implementation.
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
