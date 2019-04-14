use crate::matrix::{Matrix, Module};

//pub struct Renderer {
    //quiet_zone: bool,
//}

//pub trait Renderer {
    //fn quiet_zone(&mut self, v: bool);
//}


//pub struct StringRenderer {
    //quiet_zone: bool,
    
//}

// TODO
// Set module size/dimensions
// Set light/dark colors
// Set quiet zone
pub fn to_string(matrix: &Matrix) -> String {
    let mut res = String::with_capacity(matrix.size * matrix.size);
    for y in 0..matrix.size {
        let mut s = String::with_capacity(matrix.size + 1);
        for x in 0..matrix.size {
            let c = if matrix.is_dark(x, y) {
                '#'
            } else {
                '.'
            };
            s.push(c);
        }
        s.push('\n');
        res.push_str(&s);
    }
    res
}

pub fn to_dbg_string(matrix: &Matrix) -> String {
    let mut res = String::with_capacity(matrix.size * matrix.size);
    res.push('\n');
    for y in 0..matrix.size {
        let mut s = String::with_capacity(matrix.size + 1);
        for x in 0..matrix.size {
            let c = match matrix.get(x, y) {
                Module::Unknown => '?',
                Module::Reserved => '*',
                Module::Function(true) => '#',
                Module::Function(false) => '.',
                Module::Data(true) => 'X',
                Module::Data(false) => '-',
            };
            s.push(c);
        }
        s.push('\n');
        res.push_str(&s);
    }
    res
}

pub fn to_svg(matrix: &Matrix) -> String {
    let cell_w = 10;

    let mut res = String::from(format!(
"<?xml version=\"1.0\" standalone=\"yes\"?>
<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\"
     viewBox=\"0 0 {w} {h}\" shape-rendering=\"crispEdges\">
<rect x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\" fill=\"#fff\"/>
<path fill=\"#000\" d=\"",
    w = cell_w * (matrix.size + 8),
    h = cell_w * (matrix.size + 8)));

    for y in 0..matrix.size {
        for x in 0..matrix.size {
            if matrix.is_dark(x, y) {
                res.push_str(format!("M{x} {y}h{w}v{h}H{x}V{y}",
                                     x = (x + 4) * cell_w,
                                     y = (y + 4) * cell_w,
                                     w = cell_w,
                                     h = cell_w).as_str());
            }
        }
    }
    res.push_str("\"/></svg>");
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::QrBuilder;
    use crate::version::Version;
    use crate::ec::ECLevel;

    #[test]
    fn tmp() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.build("HELLO WORLD", &ECLevel::Q);
        println!("{}", to_svg(&builder.matrix));
        assert!(false);
    }
}


//pub fn dbg_print(&self) {
    //let size = self.version.size();

    //for y in 0..size {
        //let mut s = String::with_capacity(size + 1);
        //for x in 0..size {
            //s.push(self.to_dbg_char(x, y));
        //}
        //println!("{}", s);
    //}
//}

//pub fn to_dbg_char(&self, x: usize, y: usize) -> char {
    //if self.is_fun(x, y) {
        //if self.is_set(x, y) { '.' } else { '#' }
    //} else {
        //if self.is_set(x, y) { '1' } else { ' ' }
    //}
//}

//fn to_char(&self, x: usize, y: usize) -> char {
    //if self.is_set(x, y) { '.' } else { '#' }
//}
