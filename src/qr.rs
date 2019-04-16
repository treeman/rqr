//! Provides a simple and safe API.

use crate::version::Version;
use crate::ec::ECLevel;
use crate::mode::Mode;
use crate::mask::Mask;
use crate::matrix::Matrix;
use crate::builder::*;

/// The QR code.
///
/// Encapsulates a matrix, the 2D-grid containing the QR modules
/// and some information about the QR code.
#[derive(Debug, PartialEq, Eq)]
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
    pub mask: Mask,
}

impl Qr {
    /// Create a new QR from a string.
    pub fn new(s: &str) -> Result<Qr, Error> {
        QrBuilder::new().into(s)
    }

    /// Create a new QR with specified error correction.
    pub fn with_ecl(s: &str, ecl: ECLevel) -> Result<Qr, Error> {
        QrBuilder::new().ecl(ecl).into(s)
    }

    /// Create a new QR with a specified version.
    pub fn with_version(s: &str, v: Version) -> Result<Qr, Error> {
        QrBuilder::new().version(v).into(s)
    }

    /// Returns the size of the QR code.
    pub fn size(&self) -> usize {
        self.version.size()
    }
}

