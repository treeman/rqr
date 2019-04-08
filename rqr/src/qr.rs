use crate::version::Version;
use crate::ec::ECLevel;
use crate::mode::Mode;

use bitvec::*;

/// The QR code.
pub struct Qr {
    /// Version of the QR code.
    version: Version,

    /// Error correction level.
    ec: ECLevel,

    /// Encoding mode.
    mode: Mode,

    /// The modules, 0 represents black.
    modules: BitVec,
}

impl Qr {
    /// Returns the size of the QR code.
    pub fn size(&self) -> usize {
        self.version.size()
    }
}

