use regex::Regex;
use bitvec::*;
use lazy_static::lazy_static;

/// Encoding modes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Numeric = 0, // 0..9
    Alphanumeric, // 0..9, A-Z and $%*+-./: and space
    Byte, // ISO-8859-1 character set
    //ECI, // specifies the character set directly (like UTF-8)
    //Kanji, // more efficient storage than ECI
}

impl Mode {
    /// Create Mode from string, decide from content.
    pub fn from_str(s: &str) -> Mode {
        if NUMERIC_RX.is_match(s) {
            Mode::Numeric
        } else if ALPHANUMERIC_RX.is_match(s) {
            Mode::Alphanumeric
        } else {
            Mode::Byte
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

