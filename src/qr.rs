use crate::version::Version;
use crate::ec::ECLevel;
use crate::mode::Mode;
use crate::matrix::Matrix;

/// The QR code.
pub struct Qr {
    /// Version of the QR code.
    pub version: Version,

    /// Error correction level.
    pub ecl: ECLevel,

    /// Encoding mode.
    pub mode: Mode,

    /// The modules.
    pub matrix: Matrix,

    /// The applied mask, 0 to 7.
    pub mask: usize,
}

impl Qr {
    // TODO
    // Should be able to leave out error correction level and version,
    // but can supply them if we want to.

    //pub fn new(s: &str, ecl: &ECLevel) -> Qr {
    //}
    //pub fn with_version(s: &str, v: &Version) -> Qr {

    //}

    /// Returns the size of the QR code.
    pub fn size(&self) -> usize {
        self.version.size()
    }
}

