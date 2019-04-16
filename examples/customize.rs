use rqr::{StringRenderer, QrBuilder, ECLevel, Version, Mask, Mode};

fn main() {
    // You can specify more in detail if you interface against the builder.
    // Normally the only one you should specify is the error correction level,
    // the other values are inferred optimally.
    //
    // It's possible to gain even more fine grain control, like adding raw
    // bits, via the builder or against the matrix directly.
    let qr = QrBuilder::new()
        .ecl(ECLevel::L)
        .version(Version::new(3))
        .mask(Mask::new(0))
        .mode(Mode::Alphanumeric)
        .into("1234567890")
        .unwrap();
    let s = StringRenderer::new().render(&qr);
    println!("{}", s);
}

