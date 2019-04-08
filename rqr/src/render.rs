use crate::builder;

use bitvec::*;

pub fn to_string(modules: &BitVec, size: usize) -> String {
    let mut res = String::with_capacity(size * size);
    for y in 0..size {
        let mut s = String::with_capacity(size + 1);
        for x in 0..size {
            let c = if modules[builder::index(x, y, size)] {
                '.'
            } else {
                '#'
            };
            s.push(c);
        }
        s.push('\n');
        res.push_str(&s);
    }
    res
}

//fn to_char(&self, x: usize, y: usize) -> char {
    //if self.is_set(x, y) { '.' } else { '#' }
//}
