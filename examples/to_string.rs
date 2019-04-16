use rqr::{Qr, StringRenderer, ECLevel};

fn main() {
    // FIXME there's some error here with ECLevel::M
    let qr = Qr::with_ecl("https://github.com/treeman/rqr", ECLevel::H).unwrap();
    let s = StringRenderer::new()
        .light_module(' ')
        .dark_module('#')
        .module_dimensions(2, 1)
        .quiet_zone(false)
        .render(&qr);
    println!("{}", s);
}

