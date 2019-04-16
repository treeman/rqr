//! Encoding modes for a QR code.

use regex::Regex;
use bitvec::*;
use lazy_static::lazy_static;

/// Encoding modes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    /// Numeric mode can encode numbers, 0 to 9.
    Numeric = 0,
    /// Alphanumeric mode can encode numbers, A-Z (only uppercase),
    /// the characters $%*+-./: and space.
    Alphanumeric,
    /// Byte mode supports the ISO-8859-1 character set.
    /// It is possible to use this mode to encode unicode,
    /// but it depends heavily on the reader if it's supported or not.
    Byte,
    //ECI, // specifies the character set directly (like UTF-8)
    //Kanji, // more efficient storage than ECI
}

impl Mode {
    /// Create Mode from string, decide from content.
    pub fn from_str(s: &str) -> Mode {
        if Mode::in_numeric(s) {
            Mode::Numeric
        } else if Mode::in_alphanumeric(s) {
            Mode::Alphanumeric
        } else if Mode::in_byte(s) {
            Mode::Byte
        } else {
            // Should never happen.
            panic!("Unsupported mode for string {}", s);
        }
    }

    /// Is this a valid mode for a string?
    pub fn matches(&self, s: &str) -> bool {
        match self {
            Mode::Numeric => Mode::in_numeric(s),
            Mode::Alphanumeric => Mode::in_alphanumeric(s),
            Mode::Byte => Mode::in_byte(s),
        }
    }

    /// BitVec representation.
    pub fn to_bitvec(&self) -> BitVec {
        match self {
            Mode::Numeric => bitvec![0, 0, 0, 1],
            Mode::Alphanumeric => bitvec![0, 0, 1, 0],
            Mode::Byte => bitvec![0, 1, 0, 0],
        }
    }

    /// Returns true if contents can be represented by the numeric mode.
    pub fn in_numeric(s: &str) -> bool {
        NUMERIC_RX.is_match(s)
    }

    /// Returns true if contents can be represented by the alphanumeric mode.
    pub fn in_alphanumeric(s: &str) -> bool {
        ALPHANUMERIC_RX.is_match(s)
    }

    /// Returns true if contents can be represented by the byte mode.
    pub fn in_byte(_s: &str) -> bool {
        true
    }
}

lazy_static! {
    static ref NUMERIC_RX: Regex = Regex::new(r"^[0-9]+$").unwrap();
    static ref ALPHANUMERIC_RX: Regex = Regex::new(r"^[0-9A-Z$%*+-./: ]+$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        assert_eq!(Mode::from_str("0123456789"),
                   Mode::Numeric);
        assert_eq!(Mode::from_str("ABCXYZ 0123456789$%*+-./:"),
                   Mode::Alphanumeric);
        assert_eq!(Mode::from_str("☃"),
                   Mode::Byte);
    }

    #[test]
    fn internal() {
        assert_eq!(Mode::Numeric.to_bitvec(), bitvec![0, 0, 0, 1]);
    }
}

