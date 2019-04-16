use rqr::{Qr, StringRenderer};

fn main() {
    let qr = Qr::new("HELLO WORLD").unwrap();
    let s = StringRenderer::new().render(&qr);
    println!("{}", s);
}

