extern crate rqr;

use rqr::builder::QrBuilder;
use rqr::version::Version;
use rqr::ec::ECLevel;
use rqr::render::*;
use rqr::qr::Qr;
use rqr::mode::Mode;
use rqr::mask::Mask;

fn main() {
    //let mut builder = QrBuilder::new()
        //.version(Version::new(1))
        //.ecl(ECLevel::Q);
    //builder.add_all("HELLO WORLD").unwrap();
    //let s = SvgRenderer::new()
        //.light_module(Color::new(229, 189, 227))
        //.dark_module(Color::new(119, 0, 0))
        //.dimensions(200, 200)
        //.render(&builder.matrix);
    //println!("{}", s);

    let qr = Qr::new("HELLO WORLD").unwrap();
    let s = StringRenderer::new().render(&qr);
    println!("{}", s);
    //let s = SvgRenderer::new()
        //.light_module(Color::new(229, 189, 227))
        //.dark_module(Color::new(119, 0, 0))
        //.dimensions(200, 200)
        //.render(&builder.matrix);

    // TODO this should be an interface.
    let qr2: Qr = QrBuilder::new()
        .ecl(ECLevel::L)
        .version(Version::new(3))
        .mask(Mask::new(2))
        .mode(Mode::Byte)
        .into("HELLO WORLD")
        .unwrap();
    let s = StringRenderer::new().render(&qr2);
    println!("{}", s);
}

