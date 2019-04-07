use crate::version::Version;
use crate::ec_encoding::ECLevel;
use bitvec::*;

pub struct Qr {
    version: Version,

    size: usize,

    // Zero is black.
    modules: BitVec,

    // If set it marks the bit as a function.
    functions: BitVec,
}

impl Qr {
    pub fn new(v: &Version, _ecl: &ECLevel) -> Qr {
        let size = v.size();

        let mut res = Self {
            version: (*v).clone(),
            size: size,
            modules: bitvec![0; size * size],
            functions: bitvec![0; size * size],
        };

        res.add_finders();
        res.add_alignments();
        res.add_timing_patterns();
        res.add_dark_module();
        res.add_reserved_areas();

        // Reserved areas (which aren't considered during masking)
        // Data bits

        res
    }

    fn add_finders(&mut self) {
        let size = self.size;

        self.add_finder(0, 0);
        self.add_separator(0, 7, 7, 7);
        self.add_separator(7, 0, 7, 7);

        self.add_finder(size - 7, 0);
        self.add_separator(size - 8, 7, size - 1, 7);
        self.add_separator(size - 8, 0, size - 8, 7);

        self.add_finder(0, size - 7);
        self.add_separator(0, size - 8, 7, size - 8);
        self.add_separator(7, size - 8, 7, size - 1);
    }


    // x and y specifies the top left corner
    fn add_finder(&mut self, x: usize, y: usize) {
        self.set_rect_outline(x + 1, y + 1, 5);
        self.mark_fun_rect(x, y, 7);
    }

    fn add_separator(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for a in x0..x1 + 1 {
            for b in y0..y1 + 1 {
                self.set_fun(a, b);
                self.set(a, b);
            }
        }
    }

    fn add_alignments(&mut self) {
        let locations = ALIGNMENT_LOCATIONS[self.version.index()];
        for x in locations.iter() {
            for y in locations.iter() {
                self.try_add_alignment(*x, *y);
            }
        }
    }

    // x and y specifies the center point
    fn try_add_alignment(&mut self, cx: usize, cy: usize) {
        let x = cx - 2;
        let y = cy - 2;
        if !self.any_fun_in_rect(x, y, 5) {
            self.set_rect_outline(x + 1, y + 1, 3);
            self.mark_fun_rect(x, y, 5);
        }
    }

    fn add_timing_patterns(&mut self) {
        let offset = 6;
        for i in offset..self.size - offset {
            let v = i % 2 == 1;
            self.set_timing(i, offset, v);
            self.set_timing(offset, i, v);
        }
    }

    fn set_timing(&mut self, x: usize, y: usize, v: bool) {
        // Timing patterns should always overlap with finders and alignment modules.
        if self.is_fun(x, y) {
            assert_eq!(self.is_set(x, y), v, "timing overlap {},{}", x, y);
        }

        self.set_fun(x, y);
        if v {
            self.set(x, y);
        }
    }

    fn add_dark_module(&mut self) {
        let x = 8;
        let y = 4 * self.version.v() + 9;
        self.set_fun(x, y);
    }

    fn add_reserved_areas(&mut self) {
        let size = self.size;

        self.reserve_rect(0, 8, 8, 8);
        self.reserve_rect(8, 0, 8, 8);

        self.reserve_rect(size - 8, 8, size - 1, 8);

        self.reserve_rect(8, size - 8, 8, size - 1);

        // Versions 7 and larger needs two areas for version information.
        if self.version.v() >= 7 {
            self.reserve_rect(0, size - 11, 6, size - 9);
            self.reserve_rect(size - 11, 0, size - 9, 6);
        }
    }

    fn reserve_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for a in x0..x1 + 1 {
            for b in y0..y1 + 1 {
                self.set_fun(a, b);
            }
        }
    }

    // Return true if any module in a rect is marked as function
    fn any_fun_in_rect(&self, x: usize, y: usize, w: usize) -> bool {
        for b in y..y + w {
            for a in x..x + w {
                if self.is_fun(a, b) {
                    return true;
                }
            }
        }
        false
    }

    fn set_rect_outline(&mut self, x: usize, y: usize, w: usize) {
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

    // FIXME could combine with reserve_rect instead?
    fn mark_fun_rect(&mut self, x: usize, y: usize, w: usize) {
        for b in y..y + w {
            for a in x..x + w {
                self.set_fun(a, b);
            }
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        let i = self.index(x, y);
        self.modules.set(i, true);
    }

    fn set_fun(&mut self, x: usize, y: usize) {
        let i = self.index(x, y);
        self.functions.set(i, true);
    }

    fn is_set(&self, x: usize, y: usize) -> bool {
        let i = self.index(x, y);
        self.modules[i]
    }

    fn is_fun(&self, x: usize, y: usize) -> bool {
        let i = self.index(x, y);
        self.functions[i]
    }

    fn index(&self, x: usize, y:usize) -> usize {
        assert!(x < self.size);
        assert!(y < self.size);
        self.size * y + x
    }

    pub fn to_string(&self) {
        let size = self.version.size();

        for y in 0..size {
            let mut s = String::with_capacity(size + 1);
            for x in 0..size {
                s.push(self.to_char(x, y));
            }
            println!("{}", s);
        }
    }

    fn to_char(&self, x: usize, y: usize) -> char {
        if self.is_fun(x, y) {
            if self.is_set(x, y) { '.' } else { '#' }
        } else {
            if self.is_set(x, y) { '1' } else { ' ' }
        }
    }

    // TODO data iterator
    // Is a better idea if we can automatically avoid function data
}

#[derive(Debug, Clone)]
enum ModuleType {
    Free,
    Finder,
    Timing,
    Dark,
    Separator,
    Alignment,
    Reserved,
    Data,
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
        //let qr = Qr::new(&Version::new(1), &ECLevel::Q);
        //let qr = Qr::new(&Version::new(2), &ECLevel::Q);
        let qr = Qr::new(&Version::new(7), &ECLevel::Q);
        //let qr = Qr::new(&Version::new(8), &ECLevel::Q);
        qr.to_string();
        assert!(false);
    }
}


static ALIGNMENT_LOCATIONS: [&[usize]; 40] = [
    &[],
    &[6, 18],
    &[6, 22],
    &[6, 26],
    &[6, 30],
    &[6, 34],
    &[6, 22, 38],
    &[6, 24, 42],
    &[6, 26, 46],
    &[6, 28, 50],
    &[6, 30, 54],
    &[6, 32, 58],
    &[6, 34, 62],
    &[6, 26, 46, 66],
    &[6, 26, 48, 70],
    &[6, 26, 50, 74],
    &[6, 30, 54, 78],
    &[6, 30, 56, 82],
    &[6, 30, 58, 86],
    &[6, 34, 62, 90],
    &[6, 28, 50, 72, 94],
    &[6, 26, 50, 74, 98],
    &[6, 30, 54, 78, 102],
    &[6, 28, 54, 80, 106],
    &[6, 32, 58, 84, 110],
    &[6, 30, 58, 86, 114],
    &[6, 34, 62, 90, 118],
    &[6, 26, 50, 74, 98, 122],
    &[6, 30, 54, 78, 102, 126],
    &[6, 26, 52, 78, 104, 130],
    &[6, 30, 56, 82, 108, 134],
    &[6, 34, 60, 86, 112, 138],
    &[6, 30, 58, 86, 114, 142],
    &[6, 34, 62, 90, 118, 146],
    &[6, 30, 54, 78, 102, 126, 150],
    &[6, 24, 50, 76, 102, 128, 154],
    &[6, 28, 54, 80, 106, 132, 158],
    &[6, 32, 58, 84, 110, 136, 162],
    &[6, 26, 54, 82, 110, 138, 166],
    &[6, 30, 58, 86, 114, 142, 170]];
