use regex::Regex;

/// Encoding modes.
#[derive(Debug, PartialEq)]
pub enum Mode {
    Numeric(Vec<u8>), // 0..9
    Alphanumeric(Vec<u8>), // 0..9, A-Z and $%*+-./: and space
    Byte(Vec<u8>), // ISO-8859-1 character set
    //ECI, // specifies the character set directly (like UTF-8)
    //Kanji, // more efficient storage than ECI
}

impl Mode {
    pub fn new(s: &str) -> Mode {
        let numeric = Regex::new(r"^[0-9]+$").unwrap();
        let alphanumeric = Regex::new(r"^[0-9A-Z$%*+-./: ]+$").unwrap();

        if numeric.is_match(s) {
            Mode::Numeric(s.bytes()
                           .map(convert_numeric)
                           .collect())
        } else if alphanumeric.is_match(s) {
            Mode::Alphanumeric(s.chars()
                                 .map(convert_alphanumeric)
                                 .collect())
        } else {
            Mode::Byte(s.bytes()
                        .collect())
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Mode::Numeric(v) => v.len(),
            Mode::Alphanumeric(v) => v.len(),
            Mode::Byte(v) => v.len(),
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
    fn mode_creation() {
        assert_eq!(Mode::new("0123456789"),
                   Mode::Numeric(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(Mode::new("ABCXYZ 0123456789$%*+-./:"),
                   Mode::Alphanumeric(vec![10, 11, 12, 33, 34, 35, 36,
                                           0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                                           37, 38, 39, 40, 41, 42, 43, 44]));
        assert_eq!(Mode::new("abc"),
                   Mode::Byte(vec![97, 98, 99]));
        // No ECI support yet.
        assert_eq!(Mode::new("☃"),
                   Mode::Byte(vec![0b11100010, 0b10011000, 0b10000011]));
    }
}

