use crate::mode::Mode;
use crate::ec_encoding::ECLevel;

/// QR code version, defines the size
#[derive(Debug)]
pub struct Version(usize);

impl Version {
    pub fn new(v: usize) -> Version {
        assert!(v >= 1 && v <= 40);
        Version(v)
    }

    pub fn v(&self) -> usize {
        self.0
    }

    // FIXME move out of impl?
    pub fn minimal(_mode: &Mode, _e: &ECLevel) -> Version {
        // TODO minimal version calculation
        Version(1)
    }

    pub fn index(&self) -> usize {
        (self.0 - 1)
    }

    pub fn size(&self) -> usize {
        (((self.0 - 1) * 4) + 21)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(Version::new(32).size(), 145);
    }
}
