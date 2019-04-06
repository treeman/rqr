use crate::version::Version;
use crate::ec_encoding::ECLevel;
use bitvec::*;

pub struct Qr {
    version: Version,

    size: usize,

    // FIXME storing as a raw BitVec is efficient,
    // but debugging is easier if we have extra metadata.
    // Should we instead store as an enum type?
    modules: BitVec,
}

impl Qr {
    pub fn new(v: &Version, ecl: &ECLevel) -> Qr {
        let size = v.size();

        let mut res = Self {
            version: (*v).clone(),
            size: size,
            modules: bitvec![0; size * size]
        };

        // Finder patterns
        res.add_finder(0, 0);
        res.add_finder(size - 7, 0);
        res.add_finder(0, size - 7);
        res.add_finder(size - 7, size - 7);

        // Separators

        // Alignment patterns

        // Timing patterns

        // Dark modules

        // Reserved areas (which aren't considered during masking)

        // Data bits

        res
    }

    fn add_finder(&mut self, x: usize, y: usize) {
        self.set_rect(x + 1, y + 1, 5);
    }

    fn set_rect(&mut self, x: usize, y: usize, w: usize) {
        // Above and below
        for a in x..x + w {
            self.set(a, y);
            self.set(a, y + w - 1);
        }
        // Left and right
        for b in y + 1..y + w - 1 {
            self.set(x, b);
            self.set(x + w - 1, b);
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        let i = self.size * y + x;
        self.modules.set(i, true);
    }

    fn is_set(&self, x: usize, y: usize) -> bool{
        let i = self.size * y + x;
        self.modules[i]
    }

    pub fn to_string(&self) {
        let size = self.version.size();

        for y in 0..size {
            let mut s = String::with_capacity(size + 1);
            for x in 0..size {
                let c = if self.is_set(x, y) { '#' } else { '.' };
                s.push(c);
            }
            println!("{}", s);
        }
    }

    // TODO data iterator
    // Is a better idea if we can automatically avoid function data
}

//struct QrPrintIt<'a> {
    //i: usize,
    //data: &'a BitVec,
//}

//impl<'a> QrPrintIt<'a> {
    //fn new(bv: &'a BitVec) -> Self {
        //QrPrintIt {
            //i: 0,
            //data: bv
        //}
    //}
//}

//impl<'a> Iterator for QrPrintIt<'a> {
    //type Item = bool;
    //fn next(&mut self) -> Option<Self::Item> {
        //if self.i < self.data.len() {
            //let res = self.data[self.i];
            //self.i += 1;
            //Some(res)
        //} else {
            //None
        //}
    //}
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placement() {
        let qr = Qr::new(&Version::new(1), &ECLevel::Q);
        qr.to_string();
        assert!(false);
    }
}

