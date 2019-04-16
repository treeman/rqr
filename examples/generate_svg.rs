use rqr::{Qr, SvgRenderer, Color};

fn main() {
    let qr = Qr::new("HELLO WORLD").unwrap();
    let s = SvgRenderer::new()
        .light_module(Color::new(229, 189, 227))
        .dark_module(Color::new(119, 0, 0))
        .dimensions(200, 200)
        .render(&qr);
    println!("{}", s);
}

