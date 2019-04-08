use regex::Regex;

/// Encoding modes.
#[derive(Debug, PartialEq)]
pub enum Mode {
    Numeric, // 0..9
    Alphanumeric, // 0..9, A-Z and $%*+-./: and space
    Byte, // ISO-8859-1 character set
    //ECI, // specifies the character set directly (like UTF-8)
    //Kanji, // more efficient storage than ECI
}

impl Mode {
    /// Create Mode from string, decide from content.
    pub fn from_str(s: &str) -> Mode {
        let numeric = Regex::new(r"^[0-9]+$").unwrap();
        let alphanumeric = Regex::new(r"^[0-9A-Z$%*+-./: ]+$").unwrap();

        if numeric.is_match(s) {
            Mode::Numeric
        } else if alphanumeric.is_match(s) {
            Mode::Alphanumeric
        } else {
            Mode::Byte
        }
    }

    /// Convert string representation to bytes.
    pub fn to_bytes(&self, s: &str) -> Vec<u8> {
        match self {
            Mode::Numeric =>
                s.bytes().map(convert_numeric).collect(),
            Mode::Alphanumeric =>
                s.chars().map(convert_alphanumeric).collect(),
            Mode::Byte =>
                s.bytes().collect(),
        }
    }
}


fn convert_numeric(b: u8) -> u8 {
    b - 48
}

fn convert_alphanumeric(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'I' => 18,
        'J' => 19,
        'K' => 20,
        'L' => 21,
        'M' => 22,
        'N' => 23,
        'O' => 24,
        'P' => 25,
        'Q' => 26,
        'R' => 27,
        'S' => 28,
        'T' => 29,
        'U' => 30,
        'V' => 31,
        'W' => 32,
        'X' => 33,
        'Y' => 34,
        'Z' => 35,
        ' ' => 36,
        '$' => 37,
        '%' => 38,
        '*' => 39,
        '+' => 40,
        '-' => 41,
        '.' => 42,
        '/' => 43,
        ':' => 44,
        _ => panic!("Unsupported alphanumeric '{}'", c),
    }
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
    fn to_bytes() {
        assert_eq!(Mode::Numeric.to_bytes("0123456789"),
                   vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(Mode::Alphanumeric.to_bytes("ABCXYZ 0123456789$%*+-./:"),
                   vec![10, 11, 12, 33, 34, 35, 36,
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                        37, 38, 39, 40, 41, 42, 43, 44]);
        assert_eq!(Mode::Byte.to_bytes("☃"),
                   vec![0b11100010, 0b10011000, 0b10000011]);
    }
}

