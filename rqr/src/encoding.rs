//mod mode;
use crate::mode::Mode;

/// Error correction level
#[derive(Debug)]
pub enum ECLevel {
    L, // Recovers 7% of data
    M, // Recovers 15% of data
    Q, // Recovers 25% of data
    H, // Recovers 30% of data
}


/// QR code version, defines the size
#[derive(Debug)]
pub struct Version(u16);

impl Version {
    pub fn new(v: u16) -> Version {
        assert!(v >= 1 && v <= 40);
        Version(v)
    }

    pub fn minimal(mode: &Mode, e: &ECLevel) -> Version {
        // TODO minimal version calculation
        Version(1)
    }
}

#[derive(Debug)]
pub struct Encoding {
    // Maybe use a BitVec instead
    mode: u8,
    char_count: Vec<u8>,
    data: Vec<u8>,
    pad: Vec<u8>,
}

impl Encoding {
    pub fn new(mode: &mode, v: &Version) -> Encoding {

    }
}

