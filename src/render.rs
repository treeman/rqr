use crate::matrix::Matrix;

pub fn to_string(matrix: &Matrix) -> String {
    let mut res = String::with_capacity(matrix.size * matrix.size);
    for y in 0..matrix.size {
        let mut s = String::with_capacity(matrix.size + 1);
        for x in 0..matrix.size {
            let c = if matrix.is_set(x, y) {
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
