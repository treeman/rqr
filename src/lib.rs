// Just during development phase
#![allow(dead_code)]

pub mod builder;
pub mod data;
pub mod ec;
pub mod info;
pub mod mask;
pub mod matrix;
pub mod mode;
pub mod qr;
pub mod render;
pub mod version;

//use mode::Mode;
//use version::Version;
//use ec::ECLevel;

//pub fn qr(x: &str) {
    //println!("qr {}", x);

    //// Data analysis to determine the most efficient mode
    ////    numeric mode
    ////    alphanumeric mode (no lowercase)
    ////    UTF-8
    ////    Kanji
    ////
    //let mode = Mode::new(x);
    //println!("m: {:?}", mode);

    //// Data Encoding
    //// 1. Choose error correction level
    ////    L     recovers 7% of data
    ////    M     recovers 15% of data
    ////    Q     recovers 25% of data
    ////    H     recovers 30% of data
    //// 2. Determine smallest version of the data
    ////    Called versions, 40 versions available
    ////    See table for limits
    //let ecl = ECLevel::M;
    //let v = Version::minimal(&mode, &ecl);
    //println!("correction level: {:?} version: {:?}", ecl, v);

    //// 3. Add mode indicator
    //// 4. Add character count indicator
    //// 5. Encode using selected mode
    //// 6. Break up into 8-bit codewords and pad if necessary
    ////
    //// Error correction coding
    ////
    //// Structure final message
    ////
    //// Module placement
    ////
    //// Data masking
    ////
    //// Format & version information

    ////println!("Got {}", x);
    ////println!("Got {:?}", Mode::Numeric);

    ////println!("chars:");
    ////for c in x.chars() {
        ////println!("{}", c);
    ////}

    ////println!("bytes:");
    ////for b in x.bytes() {
        ////println!("{}", b);
    ////}
    ////println!("{:?}", b"Hello");
//}

