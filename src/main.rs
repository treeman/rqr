extern crate rqr;

use rqr::builder::QrBuilder;
use rqr::version::Version;
use rqr::ec::ECLevel;
use rqr::render::*;

fn main() {
    let mut builder = QrBuilder::new().version(Version::new(1));
    builder.add_all("HELLO WORLD", &ECLevel::Q);
    let s = SvgRenderer::new()
        .light_module(Color::new(229, 189, 227))
        .dark_module(Color::new(119, 0, 0))
        .dimensions(200, 200)
        .render(&builder.matrix);
    println!("{}", s);

    // TODO this should be an interface.
    //let qr: Qr = QrBuilder::new()
        //.ecl(&ECLevel::Q)
        //.version(&Version::new(1))
        //.mask(2)
        //.mode(Mode::Byte)
        //.into();
}

