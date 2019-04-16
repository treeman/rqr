use crate::data;
use crate::ec::ECLevel;
use crate::ec;
use crate::info;
use crate::mask;
use crate::mode::Mode;
use crate::matrix::{Matrix, Module};
use crate::render;
use crate::version::Version;
use crate::qr::Qr;

use bitvec::*;

/// Builder for a QR code.
pub struct QrBuilder {
    // Settings during build.
    //pub version: Option<Version>,
    pub version: Version,
    pub mask: Option<usize>,
    pub ecl: Option<ECLevel>,
    pub mode: Option<Mode>,

    // Resulting matrix.
    pub matrix: Matrix,

    // Build status.
    has_fun_patterns: bool,
    has_data: bool,
    is_masked: bool,
    has_info: bool,
}

impl QrBuilder {
    pub fn new(v: &Version) -> QrBuilder {
        QrBuilder {
            version: *v,
            matrix: Matrix::new(v.size()),
            mask: None,
            ecl: None,
            mode: None,
            has_fun_patterns: false,
            has_data: false,
            is_masked: false,
            has_info: false,
        }
    }

    /// Build all elements and generate a QR code.
    pub fn build_qr(mut self, s: &str, ecl: &ECLevel) -> Qr {
        self.add_all(s, ecl);
        self.into_qr()
    }

    // TODO Option or Result
    pub fn into_qr(self) -> Qr {
        Qr {
            version: self.version,
            matrix: self.matrix,
            ecl: self.ecl.unwrap(),
            mode: self.mode.unwrap(),
            mask: self.mask.unwrap(),
        }
    }

    /// Add all elements of a QR code.
    pub fn add_all(&mut self, s: &str, ecl: &ECLevel) {
        self.add_fun_patterns();
        self.add_data(s, ecl);
        self.mask();
        self.add_info();
    }

    /// Add function patterns.
    pub fn add_fun_patterns(&mut self) {
        self.add_finders();
        self.add_alignments();
        self.add_timing_patterns();
        self.add_dark_module();
        self.add_reserved_areas();
    }

    /// Add data.
    pub fn add_data(&mut self, s: &str, ecl: &ECLevel) {
        self.ecl = Some(*ecl);

        let (mode, v) = data::encode(s, &self.version, ecl);
        self.mode = Some(mode);

        let v = ec::add(v, &self.version, ecl);
        self.add_raw_data(&v);
    }

    /// Add raw data.
    pub fn add_raw_data(&mut self, v: &BitVec) {
        let mut vi = 0;
        for (x, y) in ZigZagIt::new(self.matrix.size) {
            if self.matrix.is_fun(x, y) { continue; }
            self.matrix.set_data(x, y, v[vi]);
            vi += 1;
        }
        assert_eq!(vi, v.len());
    }

    /// Apply mask.
    pub fn mask(&mut self) {
        let (mask, masked) = mask::mask(&self.matrix);
        self.mask = Some(mask);
        self.matrix = masked;
    }

    /// Mask using
    pub fn mask_with(&mut self, m: usize) {
        assert!(m <= 7);
        self.mask = Some(m);
        self.matrix = mask::apply_mask(m, &self.matrix);
    }

    /// Add info.
    pub fn add_info(&mut self) {
        self.add_format_info();
        self.add_version_info();
    }

    /// Add format info.
    pub fn add_format_info(&mut self) {
        // Hard assumption that we have necessary data.
        let format = info::format_info(&self.ecl.unwrap(), self.mask.unwrap());
        self.add_format(&format);
    }

    /// Add version info.
    pub fn add_version_info(&mut self) {
        if let Some(v) = info::version_info(&self.version) {
            self.add_version(&v);
        }
    }

    fn add_finders(&mut self) {
        let size = self.matrix.size;

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
        self.matrix.set_square(x, y, 7, Module::Function(true));
        self.matrix.set_square_outline(x + 1, y + 1, 5, Module::Function(false));
    }

    fn add_separator(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for a in x0..x1 + 1 {
            for b in y0..y1 + 1 {
                self.matrix.set(a, b, Module::Function(false));
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
        if !self.matrix.any_in_square(x, y, 4) {
            self.matrix.set_square(x, y, 5, Module::Function(true));
            self.matrix.set_square_outline(x + 1, y + 1, 3, Module::Function(false));
        }
    }

    fn add_timing_patterns(&mut self) {
        let offset = 6;
        for i in offset..self.matrix.size - offset {
            let v = i % 2 == 0;
            self.set_timing(i, offset, v);
            self.set_timing(offset, i, v);
        }
    }

    fn set_timing(&mut self, x: usize, y: usize, v: bool) {
        // Timing patterns should always overlap with finders and alignment modules.
        if self.matrix.is_fun(x, y) {
            assert_eq!(self.matrix.is_dark(x, y), v, "timing overlap {},{}", x, y);
        }

        self.matrix.set(x, y, Module::Function(v));
    }

    fn add_dark_module(&mut self) {
        let (x, y) = self.version.dark_module_pos();
        self.matrix.set(x, y, Module::Function(true));
    }

    fn add_reserved_areas(&mut self) {
        let size = self.matrix.size;

        // Around top left finder.
        // Avoid timing pattern.
        self.reserve_rect(0, 8, 5, 8);
        self.reserve_rect(7, 8, 8, 8);
        self.reserve_rect(8, 0, 8, 5);
        self.reserve_rect(8, 7, 8, 7);

        // Top right.
        self.reserve_rect(size - 8, 8, size - 1, 8);

        // Bottom left.
        self.reserve_rect(8, size - 7, 8, size - 1);

        //// Larger versions needs two areas for version information.
        if self.version.extra_version_areas() {
            self.reserve_rect(0, size - 11, 5, size - 9);
            self.reserve_rect(size - 11, 0, size - 9, 5);
        }
    }

    fn reserve_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        assert!(!self.matrix.any_in_rect(x0, y0, x1, y1));
        self.matrix.set_rect(x0, y0, x1, y1, Module::Reserved);
    }

    fn add_format(&mut self, bv: &BitVec) {
        assert_eq!(bv.len(), 15);
        let size = self.matrix.size;

        // Info surrounding the top left finder.
        let mut iter = bv.iter();
        for x in 0..8 {
            // Avoid timing pattern.
            if x == 6 { continue; }
            self.matrix.set_fun(x, 8, iter.next().unwrap());
        }
        for y in (0..9).rev() {
            // Avoid timing pattern.
            if y == 6 { continue; }
            self.matrix.set_fun(8, y, iter.next().unwrap());
        }
        assert_eq!(iter.next(), None);

        // Half to the right of the bottom left finder.
        iter = bv.iter();
        for y in (size - 7..size).rev() {
            self.matrix.set_fun(8, y, iter.next().unwrap());
        }
        // Rest bottom of the top left finder.
        for x in (size - 8)..size {
            self.matrix.set_fun(x, 8, iter.next().unwrap());
        }
        assert_eq!(iter.next(), None);
    }

    fn add_version(&mut self, bv: &BitVec) {
        assert_eq!(bv.len(), 18);
        let size = self.matrix.size;

        // Bottom left version block.
        let mut iter = bv.iter();
        for x in 0..6 {
            for y in (size - 11)..(size - 8) {
                self.matrix.set_fun(x, y, iter.next().unwrap());
            }
        }
        assert_eq!(iter.next(), None);

        // Top right version block.
        iter = bv.iter();
        for y in 0..6 {
            for x in (size - 11)..(size - 8) {
                self.matrix.set_fun(x, y, iter.next().unwrap());
            }
        }
        assert_eq!(iter.next(), None);
    }

    /// Convert matrix to string.
    pub fn to_string(&self) -> String {
        render::to_string(&self.matrix)
    }
    pub fn to_dbg_string(&self) -> String {
        render::to_dbg_string(&self.matrix)
    }
}

// A zig-zagging iterator which moves according to the QR data specification.
// It starts in the bottom right corner and moves flows in fields 2 bits wide
// up and down.
// Inside the 2 bit flow it alternates between the right and left field.
// It also avoids the vertical timing pattern column completely,
// but it does not automatically skip function patterns.
struct ZigZagIt {
    size: usize,
    // Should we move horizontal next step?
    horizontal_next: bool,
    // Are we moving upwards?
    upwards: bool,
    // xy coordinates into the matrix.
    x: usize,
    y: usize,
    // Valid? Used as a stop criteria.
    valid: bool,
}

impl ZigZagIt {
    fn new(size: usize) -> Self {
        Self {
            size: size,
            horizontal_next: true,
            upwards: true,
            x: size - 1,
            y: size - 1,
            valid: true,
        }
    }

    fn advance(&mut self) {
        if self.horizontal_next {
            self.move_horizontally();
        } else {
            self.move_vertically();
        }
    }

    fn move_horizontally(&mut self) {
        match self.x {
            0 => self.valid = false,
            6 => self.x -= 2,
            _ => self.x -= 1,
        }
        self.horizontal_next = false;
    }

    fn move_vertically(&mut self) {
        if (self.upwards && self.y == 0) || (!self.upwards && self.y == self.size - 1) {
            // When we've reached the edge move in the other direction instead of zagging.
            self.upwards = !self.upwards;
            self.move_horizontally();
        } else {
            // Zag motion, y is inverted
            if self.upwards {
                self.y -= 1;
            } else {
                self.y += 1;
            }
            self.x += 1;
        }
        self.horizontal_next = true;
    }
}

impl Iterator for ZigZagIt {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.valid { return None; }

        let res = Some((self.x, self.y));
        self.advance();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finders() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.add_finders();
        let expected = "
#######.?????.#######
#.....#.?????.#.....#
#.###.#.?????.#.###.#
#.###.#.?????.#.###.#
#.###.#.?????.#.###.#
#.....#.?????.#.....#
#######.?????.#######
........?????........
?????????????????????
?????????????????????
?????????????????????
?????????????????????
?????????????????????
........?????????????
#######.?????????????
#.....#.?????????????
#.###.#.?????????????
#.###.#.?????????????
#.###.#.?????????????
#.....#.?????????????
#######.?????????????
";
        println!("{}", builder.to_dbg_string());
        assert_eq!(builder.to_dbg_string(), expected);
    }

    #[test]
    fn fun_patterns() {
        let mut builder = QrBuilder::new(&Version::new(3));
        builder.add_fun_patterns();
        println!("{}", builder.to_dbg_string());
        let expected = "
#######.*????????????.#######
#.....#.*????????????.#.....#
#.###.#.*????????????.#.###.#
#.###.#.*????????????.#.###.#
#.###.#.*????????????.#.###.#
#.....#.*????????????.#.....#
#######.#.#.#.#.#.#.#.#######
........*????????????........
******#**????????????********
??????.??????????????????????
??????#??????????????????????
??????.??????????????????????
??????#??????????????????????
??????.??????????????????????
??????#??????????????????????
??????.??????????????????????
??????#??????????????????????
??????.??????????????????????
??????#??????????????????????
??????.??????????????????????
??????#?????????????#####????
........#???????????#...#????
#######.*???????????#.#.#????
#.....#.*???????????#...#????
#.###.#.*???????????#####????
#.###.#.*????????????????????
#.###.#.*????????????????????
#.....#.*????????????????????
#######.*????????????????????
";
        assert_eq!(builder.to_dbg_string(), expected);
    }

    #[test]
    fn fun_patterns_large() {
        let mut builder = QrBuilder::new(&Version::new(9));
        builder.add_fun_patterns();
        //println!("{}", builder.to_dbg_string());
        let expected = "
#######.*?????????????????????????????????***.#######
#.....#.*?????????????????????????????????***.#.....#
#.###.#.*?????????????????????????????????***.#.###.#
#.###.#.*?????????????????????????????????***.#.###.#
#.###.#.*???????????????#####?????????????***.#.###.#
#.....#.*???????????????#...#?????????????***.#.....#
#######.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#.#######
........*???????????????#...#????????????????........
******#**???????????????#####????????????????********
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
????#####???????????????#####???????????????#####????
????#...#???????????????#...#???????????????#...#????
????#.#.#???????????????#.#.#???????????????#.#.#????
????#...#???????????????#...#???????????????#...#????
????#####???????????????#####???????????????#####????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
??????#??????????????????????????????????????????????
??????.??????????????????????????????????????????????
******#??????????????????????????????????????????????
******.??????????????????????????????????????????????
******#?????????????????#####???????????????#####????
........#???????????????#...#???????????????#...#????
#######.*???????????????#.#.#???????????????#.#.#????
#.....#.*???????????????#...#???????????????#...#????
#.###.#.*???????????????#####???????????????#####????
#.###.#.*????????????????????????????????????????????
#.###.#.*????????????????????????????????????????????
#.....#.*????????????????????????????????????????????
#######.*????????????????????????????????????????????
";
        assert_eq!(builder.to_dbg_string(), expected);
    }

    #[test]
    fn add_raw_data() {
        let mut builder = QrBuilder::new(&Version::new(2));
        builder.add_fun_patterns();
        let mut bv: BitVec = BitVec::new();
        for i in 0..359 {
            bv.push(i % 2 == 0);
        }
        builder.add_raw_data(&bv);

        let expected = "
#######.*X-X-X-X-.#######
#.....#.*X-X-X-X-.#.....#
#.###.#.*X-X-X-X-.#.###.#
#.###.#.*X-X-X-X-.#.###.#
#.###.#.*X-X-X-X-.#.###.#
#.....#.*X-X-X-X-.#.....#
#######.#.#.#.#.#.#######
........*X-X-X-X-........
******#**X-X-X-X-********
X-X-X-.X-X-X-X-X--X-X-X-X
X-X-X-#X-X-X-X-X--X-X-X-X
X-X-X-.X-X-X-X-X--X-X-X-X
X-X-X-#X-X-X-X-X--X-X-X-X
X-X-X-.X-X-X-X-X--X-X-X-X
X-X-X-#X-X-X-X-X--X-X-X-X
X-X-X-.X-X-X-X-X--X-X-X-X
X-X-X-#X-X-X-X-X#####-X-X
........#X-X-X--#...#-X-X
#######.*X-X-X-X#.#.#-X-X
#.....#.*X-X-X--#...#-X-X
#.###.#.*X-X-X-X#####-X-X
#.###.#.*X-X-X--X-X-X-X-X
#.###.#.*X-X-X--X-X-X-X-X
#.....#.*X-X-X--X-X-X-X-X
#######.*X-X-X--X-X-X-X-X
";
        //println!("{}", builder.to_dbg_string());
        assert_eq!(builder.to_dbg_string(), expected);
    }

    #[test]
    fn add_data() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.add_fun_patterns();
        builder.add_data("HELLO WORLD", &ECLevel::Q);
        //println!("{}", builder.to_dbg_string());
        let expected = "
#######.*XX-X.#######
#.....#.*X---.#.....#
#.###.#.*-XX-.#.###.#
#.###.#.*X-X-.#.###.#
#.###.#.*---X.#.###.#
#.....#.*XXX-.#.....#
#######.#.#.#.#######
........*X-X-........
******#**-X--********
---X-X.XX-X--X-XXX-XX
X--XXX#XXX--X----XX-X
--XXX-.--XX-------X--
--X---#----X---X-----
........#----XXX-XXXX
#######.*---X-XXXX--X
#.....#.*---XXX----X-
#.###.#.*---X--X-X-X-
#.###.#.*--------X---
#.###.#.*-XXXX-XXXX--
#.....#.*XX-X--X----X
#######.*-XXXX-XX-X--
";
        assert_eq!(builder.to_dbg_string(), expected);
    }

    #[test]
    fn format_info() {
        let mut builder = QrBuilder::new(&Version::new(1));
        // Mask 6, ECLevel Q
        builder.add_format(&bitvec![0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0]);
        let expected = "
????????.????????????
????????#????????????
????????.????????????
????????#????????????
????????#????????????
????????.????????????
?????????????????????
????????#????????????
.#.###?.#????##.##.#.
?????????????????????
?????????????????????
?????????????????????
?????????????????????
?????????????????????
????????.????????????
????????#????????????
????????#????????????
????????#????????????
????????.????????????
????????#????????????
????????.????????????
";
        //println!("{}", builder.to_dbg_string());
        assert_eq!(builder.to_dbg_string(), expected);
    }

    #[test]
    fn hello_world() {
        // Builds a final QR code which should be scannable.
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.add_all("HELLO WORLD", &ECLevel::Q);
        let expected = "
#######..--X-.#######
#.....#.#X--X.#.....#
#.###.#..X-XX.#.###.#
#.###.#.#XXXX.#.###.#
#.###.#.#X-X-.#.###.#
#.....#..X--X.#.....#
#######.#.#.#.#######
........#X-XX........
.#.####.#X--X##.##.#.
X-XXXX.X----XXXX-XXX-
--X-X-#X---X--XX-----
X-XX-X.--X-XX---XX---
XX-XXX#XXXX-XXX-XXXXX
........#---X--X-X---
#######..XX--XX--XXXX
#.....#.#-X--X--X-XXX
#.###.#.#X-X--X---XXX
#.###.#.#-XXX---X-X--
#.###.#..X----X----XX
#.....#.#XX--XXX--XX-
#######..X-X-------X-
";
        //println!("{}", builder.to_dbg_string());
        assert_eq!(builder.to_dbg_string(), expected);
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
