//! Terminal color representation.

use std::str::FromStr;

use owo_colors::{AnsiColors, DynColors, XtermColors};

use crate::error::ParseColorError;

/// A terminal color.
///
/// Wraps color values that can represent ANSI 16, ANSI 256, or TrueColor (RGB).
/// Use the named constants for basic ANSI colors, [`from_rgb`](Color::from_rgb)
/// for TrueColor, [`From<u8>`] for ANSI 256 indices, or [`FromStr`] to parse
/// from strings.
///
/// # Examples
///
/// ```
/// use craftty_ink::color::Color;
///
/// let red = Color::RED;
/// let orange = Color::from(208);
/// let custom = Color::from_rgb(0, 255, 128);
/// let parsed: Color = "#ff00ff".parse().unwrap();
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Color(pub(crate) DynColors);

impl Color {
    /// Black (`\x1b[30m`).
    pub const BLACK: Color = Color(DynColors::Ansi(AnsiColors::Black));
    /// Red (`\x1b[31m`).
    pub const RED: Color = Color(DynColors::Ansi(AnsiColors::Red));
    /// Green (`\x1b[32m`).
    pub const GREEN: Color = Color(DynColors::Ansi(AnsiColors::Green));
    /// Yellow (`\x1b[33m`).
    pub const YELLOW: Color = Color(DynColors::Ansi(AnsiColors::Yellow));
    /// Blue (`\x1b[34m`).
    pub const BLUE: Color = Color(DynColors::Ansi(AnsiColors::Blue));
    /// Magenta (`\x1b[35m`).
    pub const MAGENTA: Color = Color(DynColors::Ansi(AnsiColors::Magenta));
    /// Cyan (`\x1b[36m`).
    pub const CYAN: Color = Color(DynColors::Ansi(AnsiColors::Cyan));
    /// White (`\x1b[37m`).
    pub const WHITE: Color = Color(DynColors::Ansi(AnsiColors::White));
    /// Bright black (`\x1b[90m`).
    pub const BRIGHT_BLACK: Color = Color(DynColors::Ansi(AnsiColors::BrightBlack));
    /// Bright red (`\x1b[91m`).
    pub const BRIGHT_RED: Color = Color(DynColors::Ansi(AnsiColors::BrightRed));
    /// Bright green (`\x1b[92m`).
    pub const BRIGHT_GREEN: Color = Color(DynColors::Ansi(AnsiColors::BrightGreen));
    /// Bright yellow (`\x1b[93m`).
    pub const BRIGHT_YELLOW: Color = Color(DynColors::Ansi(AnsiColors::BrightYellow));
    /// Bright blue (`\x1b[94m`).
    pub const BRIGHT_BLUE: Color = Color(DynColors::Ansi(AnsiColors::BrightBlue));
    /// Bright magenta (`\x1b[95m`).
    pub const BRIGHT_MAGENTA: Color = Color(DynColors::Ansi(AnsiColors::BrightMagenta));
    /// Bright cyan (`\x1b[96m`).
    pub const BRIGHT_CYAN: Color = Color(DynColors::Ansi(AnsiColors::BrightCyan));
    /// Bright white (`\x1b[97m`).
    pub const BRIGHT_WHITE: Color = Color(DynColors::Ansi(AnsiColors::BrightWhite));

    /// Creates a color from RGB components.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color(DynColors::Rgb(r, g, b))
    }

    /// Normalizes Xterm indices 0–15 to their ANSI 16 equivalents.
    ///
    /// Xterm indices 0–15 are the same 16 ANSI colors by spec.
    /// Normalizing ensures that `Color::from(1) == Color::RED`.
    fn from_ansi_index(index: u8) -> Color {
        let ansi = match index {
            0 => AnsiColors::Black,
            1 => AnsiColors::Red,
            2 => AnsiColors::Green,
            3 => AnsiColors::Yellow,
            4 => AnsiColors::Blue,
            5 => AnsiColors::Magenta,
            6 => AnsiColors::Cyan,
            7 => AnsiColors::White,
            8 => AnsiColors::BrightBlack,
            9 => AnsiColors::BrightRed,
            10 => AnsiColors::BrightGreen,
            11 => AnsiColors::BrightYellow,
            12 => AnsiColors::BrightBlue,
            13 => AnsiColors::BrightMagenta,
            14 => AnsiColors::BrightCyan,
            15 => AnsiColors::BrightWhite,
            _ => unreachable!(),
        };
        Color(DynColors::Ansi(ansi))
    }
}

/// Parses a color from a string.
///
/// Supported formats:
/// - Named ANSI colors: `"red"`, `"bright cyan"`, etc.
/// - Hex RGB: `"#ff0000"`
/// - Numeric ANSI 256 index: `"202"`
impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(index) = s.parse::<u8>() {
            return Ok(Color::from(index));
        }

        DynColors::from_str(s)
            .map(Color)
            .map_err(|_| ParseColorError::new(s))
    }
}

/// Creates a color from an ANSI 256 index (0–255).
impl From<u8> for Color {
    fn from(value: u8) -> Self {
        if value < 16 {
            Color::from_ansi_index(value)
        } else {
            Color(DynColors::Xterm(XtermColors::from(value)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Constants ---

    #[test]
    fn constant_red() {
        assert_eq!(Color::RED.0, DynColors::Ansi(AnsiColors::Red));
    }

    #[test]
    fn constant_bright_cyan() {
        assert_eq!(
            Color::BRIGHT_CYAN.0,
            DynColors::Ansi(AnsiColors::BrightCyan)
        );
    }

    // --- from_rgb ---

    #[test]
    fn from_rgb() {
        assert_eq!(Color::from_rgb(255, 0, 0).0, DynColors::Rgb(255, 0, 0));
    }

    #[test]
    fn from_rgb_black() {
        assert_eq!(Color::from_rgb(0, 0, 0).0, DynColors::Rgb(0, 0, 0));
    }

    #[test]
    fn from_rgb_white() {
        assert_eq!(
            Color::from_rgb(255, 255, 255).0,
            DynColors::Rgb(255, 255, 255)
        );
    }

    // --- From<u8> ---

    #[test]
    fn from_u8_above_15() {
        assert_eq!(Color::from(202).0, DynColors::Xterm(XtermColors::from(202)));
    }

    #[test]
    fn from_u8_zero() {
        assert_eq!(Color::from(0u8), Color::BLACK);
    }

    #[test]
    fn from_u8_15() {
        assert_eq!(Color::from(15u8), Color::BRIGHT_WHITE);
    }

    #[test]
    fn from_u8_16_not_normalized() {
        assert_eq!(Color::from(16u8).0, DynColors::Xterm(XtermColors::from(16)));
    }

    #[test]
    fn from_u8_255() {
        assert_eq!(
            Color::from(255u8).0,
            DynColors::Xterm(XtermColors::from(255))
        );
    }

    // --- FromStr ---

    #[test]
    fn from_str_named() {
        let color: Color = "red".parse().unwrap();
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn from_str_bright_named() {
        let color: Color = "bright red".parse().unwrap();
        assert_eq!(color, Color::BRIGHT_RED);
    }

    #[test]
    fn from_str_hex_lower() {
        let color: Color = "#ff0000".parse().unwrap();
        assert_eq!(color, Color::from_rgb(255, 0, 0));
    }

    #[test]
    fn from_str_hex_upper() {
        let color: Color = "#FF0000".parse().unwrap();
        assert_eq!(color, Color::from_rgb(255, 0, 0));
    }

    #[test]
    fn from_str_numeric() {
        let color: Color = "202".parse().unwrap();
        assert_eq!(color, Color::from(202));
    }

    #[test]
    fn from_str_numeric_zero() {
        let color: Color = "0".parse().unwrap();
        assert_eq!(color, Color::BLACK);
    }

    #[test]
    fn from_str_numeric_255() {
        let color: Color = "255".parse().unwrap();
        assert_eq!(color, Color::from(255));
    }

    #[test]
    fn from_str_invalid() {
        let err = "banana".parse::<Color>().unwrap_err();
        assert_eq!(err.input(), "banana");
        assert_eq!(err.to_string(), r#"invalid color: "banana""#);
    }

    #[test]
    fn from_str_empty() {
        let err = "".parse::<Color>().unwrap_err();
        assert_eq!(err.input(), "");
    }

    #[test]
    fn from_str_256_out_of_range() {
        let err = "256".parse::<Color>().unwrap_err();
        assert_eq!(err.input(), "256");
    }

    // --- Normalization / PartialEq ---

    #[test]
    fn from_u8_equals_constant() {
        assert_eq!(Color::from(1u8), Color::RED);
    }

    #[test]
    fn from_str_numeric_equals_constant() {
        let color: Color = "1".parse().unwrap();
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn from_str_named_equals_constant() {
        let color: Color = "red".parse().unwrap();
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn different_colors_not_equal() {
        assert_ne!(Color::RED, Color::BLUE);
    }

    #[test]
    fn rgb_not_equal_to_ansi() {
        assert_ne!(Color::from_rgb(255, 0, 0), Color::RED);
    }
}
