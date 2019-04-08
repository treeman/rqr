use crate::mode::Mode;
use crate::ec::ECLevel;

/// QR code version, defines the size
#[derive(Debug, Clone)]
pub struct Version(usize);

impl Version {
    /// Create a new version, must be in the [1..40] range.
    pub fn new(v: usize) -> Version {
        assert!(v >= 1 && v <= 40);
        Version(v)
    }

    /// Calculate the minimal required version to hold the string
    /// in the given mode with the required error correction level.
    pub fn minimal(_s: &str, _mode: &Mode, _e: &ECLevel) -> Option<Version> {
        // TODO minimal version calculation
        Some(Version(1))
    }

    /// Return the size of the QR code.
    pub fn size(&self) -> usize {
        (((self.0 - 1) * 4) + 21)
    }

    /// Returns the required len of the char count bit representation.
    pub fn char_count_len(&self, mode: &Mode) -> usize {
        if self.0 >= 1 && self.0 <= 9 {
            match mode {
                Mode::Numeric => 10,
                Mode::Alphanumeric => 9,
                Mode::Byte => 8,
            }
        } else if self.0 <= 26 {
            match mode {
                Mode::Numeric => 12,
                Mode::Alphanumeric => 11,
                Mode::Byte => 16,
            }
        } else if self.0 <= 40 {
            match mode {
                Mode::Numeric => 14,
                Mode::Alphanumeric => 13,
                Mode::Byte => 16,
            }
        } else {
            panic!("Malformed version {}", self.0);
        }
    }

    /// Returns true if this version requires extra version areas.
    pub fn extra_version_areas(&self) -> bool {
        self.0 >= 7
    }

    /// Returns the position of the dark module.
    pub fn dark_module_pos(&self) -> (usize, usize) {
        (8, 4 * self.0 + 9)
    }

    /// Return the version value - 1, suitable for indexing.
    pub fn index(&self) -> usize {
        (self.0 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(Version::new(32).size(), 145);

        assert_eq!(Version::new(1).char_count_len(&Mode::Numeric), 10);
        assert_eq!(Version::new(40).char_count_len(&Mode::Byte), 16);
    }
}

