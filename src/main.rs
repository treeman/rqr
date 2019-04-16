extern crate rqr;
#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};
use rqr::*;

fn main() {
    let matches = App::new("rqr cli")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("type")
                .help("Output type")
                .takes_value(true)
                .possible_values(&["string", "svg"])
                .default_value("string")
                .short("t"))
        .arg(Arg::with_name("input")
                .help("String to encode")
                .index(1)
                .required(true))
        .arg(Arg::with_name("bg")
                .takes_value(true)
                .long("bg")
                .help("Background color to use for svg output"))
        .arg(Arg::with_name("fg")
                .takes_value(true)
                .long("fg")
                .help("Foreground color to use for svg output"))
        .arg(Arg::with_name("width")
                .takes_value(true)
                .long("width")
                .short("w")
                .help("Image width for svg output"))
        .get_matches();

    let s = matches.value_of("input").unwrap();

    match matches.value_of("type").unwrap() {
        "svg" => output_svg(s, &matches),
        _ => output_string(s, &matches),
    }

    //let s = matches.value_of("input").unwrap();
    //let t = matches.value_of("type").unwrap();

    ////}
    //println!("type: {}", t);
    //println!("string: {}", s);

    return;

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

fn output_svg(s: &str, matches: &ArgMatches) {
    let mut r = SvgRenderer::new();

    if let Some(bg) = matches.value_of("bg") {
        let c: Color = bg.parse().expect("bg should be a color value like '#ff0033'");
        r = r.light_module(c);
    }
    if let Some(fg) = matches.value_of("fg") {
        let c: Color = fg.parse().expect("fg should be a color value like '#ff0033'");
        r = r.dark_module(c);
    }
    if let Some(w) = matches.value_of("width") {
        let w: usize = w.parse().expect("Width must be an integer value");
        r = r.dimensions(w, w);
    }
}

fn output_string(s: &str, matches: &ArgMatches) {

}


