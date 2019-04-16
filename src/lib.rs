//! A small QR code generation project I made to dip my toes into rust again
//! after several years of absence. I was always curious on how QR codes worked
//! and it was a pretty good project to explore rust with. I followed an
//! [excellent tutorial](https://www.thonky.com/qr-code-tutorial/).
//!
//! This library supports strings encoded in numeric, alphanumeric and byte mode.
//! It supports all versions, meaning different sizes, of a standard QR code with
//! the different error correction levels.
//!
//! # QR code as string output
//!
//! ```
//! extern crate rqr;
//! use rqr::{Qr, StringRenderer};
//!
//! fn main() {
//!     let qr = Qr::new("HELLO WORLD").unwrap();
//!     let s = StringRenderer::new().render(&qr);
//!     println!("{}", s);
//! }
//! ```
//!
//! # SVG generation
//!
//! ```
//! use rqr::{Qr, SvgRenderer, Color, ECLevel};
//!
//! fn main() {
//!     let qr = Qr::with_ecl("HELLO WORLD", ECLevel::Q).unwrap();
//!     let s = SvgRenderer::new()
//!         .light_module(Color::new(229, 189, 227))
//!         .dark_module(Color::new(119, 0, 0))
//!         .dimensions(200, 200)
//!         .render(&qr);
//!     println!("{}", s);
//! }
//! ```
//!
//! # Override inferred settings
//!
//! If not provided the version and encoding modes will be inferred to fit the
//! encoded message. It's possible to override these and others:
//!
//! ```
//! use rqr::{QrBuilder, ECLevel, Version, Mask, Mode};
//!
//! let qr = QrBuilder::new()
//!     .ecl(ECLevel::L)
//!     .version(Version::new(3))
//!     .mask(Mask::new(0))
//!     .mode(Mode::Alphanumeric)
//!     .into("1234567890")
//!     .unwrap();
//! ```
//!
//! More fine grained control is provided by the builder and the underlying matrix.

pub mod builder;
pub use builder::*;

pub mod data;
pub use data::*;

pub mod ec;
pub use ec::*;

pub mod info;
pub use info::*;

pub mod mask;
pub use mask::*;

pub mod matrix;
pub use matrix::{Module, Matrix};

pub mod mode;
pub use mode::Mode;

pub mod qr;
pub use qr::Qr;

pub mod render;
pub use render::*;

pub mod version;
pub use version::Version;

