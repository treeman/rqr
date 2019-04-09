use crate::data;
use crate::ec::ECLevel;
use crate::ec;
use crate::format;
use crate::mask;
use crate::matrix::Matrix;
use crate::render;
use crate::version::Version;

/// Builder for a QR code.
pub struct QrBuilder {
    pub version: Version,
    pub matrix: Matrix,
    pub mask: Option<usize>,
}

impl QrBuilder {
    pub fn new(v: &Version) -> QrBuilder {
        QrBuilder {
            version: (*v).clone(),
            matrix: Matrix::new(v.size()),
            mask: None,
        }
    }

    /// Build.
    pub fn build(&mut self, s: &str, ecl: &ECLevel) {
        self.build_until_masking(s, ecl);
        self.build_mask();
        self.build_information(ecl);
    }

    /// Build modules before masking (and format/version info) is applied.
    /// Separated to allow testing.
    pub fn build_until_masking(&mut self, s: &str, ecl: &ECLevel) {
        self.add_finders();
        self.add_alignments();
        self.add_timing_patterns();
        self.add_dark_module();
        self.add_reserved_areas();
        self.add_string(s, ecl);
    }

    /// Build mask.
    pub fn build_mask(&mut self) {
        let (mask, masked) = mask::mask(&self.matrix);
        self.mask = Some(mask);
        self.matrix = masked;
    }

    /// Build format and version information.
    pub fn build_information(&mut self, ecl: &ECLevel) {
        let format = format::information(ecl, self.mask.unwrap());
        println!("format {:?}", format);
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
        self.matrix.set_square_outline(x + 1, y + 1, 5, true);
        self.matrix.mark_fun_square(x, y, 7);
    }

    fn add_separator(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for a in x0..x1 + 1 {
            for b in y0..y1 + 1 {
                self.matrix.mark_fun(a, b);
                self.matrix.set(a, b, true);
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
        if !self.matrix.any_fun_in_square(x, y, 4) {
            self.matrix.set_square_outline(x + 1, y + 1, 3, true);
            self.matrix.mark_fun_square(x, y, 5);
        }
    }

    fn add_timing_patterns(&mut self) {
        let offset = 6;
        for i in offset..self.matrix.size - offset {
            let v = i % 2 == 1;
            self.set_timing(i, offset, v);
            self.set_timing(offset, i, v);
        }
    }

    fn set_timing(&mut self, x: usize, y: usize, v: bool) {
        // Timing patterns should always overlap with finders and alignment modules.
        if self.matrix.is_fun(x, y) {
            assert_eq!(self.matrix.is_set(x, y), v, "timing overlap {},{}", x, y);
        }

        self.matrix.mark_fun(x, y);
        self.matrix.set(x, y, v);
    }

    fn add_dark_module(&mut self) {
        let (x, y) = self.version.dark_module_pos();
        self.matrix.mark_fun(x, y);
    }

    fn add_reserved_areas(&mut self) {
        let size = self.matrix.size;

        self.matrix.mark_fun_rect(0, 8, 8, 8);
        self.matrix.mark_fun_rect(8, 0, 8, 8);
        self.matrix.mark_fun_rect(size - 8, 8, size - 1, 8);
        self.matrix.mark_fun_rect(8, size - 8, 8, size - 1);

        // Larger versions needs two areas for version information.
        if self.version.extra_version_areas() {
            self.matrix.mark_fun_rect(0, size - 11, 6, size - 9);
            self.matrix.mark_fun_rect(size - 11, 0, size - 9, 6);
        }
    }

    fn add_string(&mut self, s: &str, ecl: &ECLevel) {
        let v = data::encode(s, &self.version, ecl);
        let v = ec::add(v, &self.version, ecl);

        let mut v_i = 0;
        for (x, y) in ZigZagIt::new(self.matrix.size) {
            if self.matrix.is_fun(x, y) { continue; }
            self.matrix.set(x, y, v[v_i]);
            v_i += 1;
        }
        assert_eq!(v_i, v.len());
    }

    pub fn to_string(&self) -> String {
        render::to_string(&self.matrix)
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
    fn before_masking() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.build_until_masking("HELLO WORLD", &ECLevel::Q);
        let expected =
"#######.#..#..#######
#.....#.#.###.#.....#
#.###.#.##..#.#.###.#
#.###.#.#.#.#.#.###.#
#.###.#.####..#.###.#
#.....#.#...#.#.....#
#######.#.#.#.#######
........#.#.#........
##########.##########
###.#....#.##.#...#..
.##...#...##.####..#.
##...#.##..#######.##
##.########.###.#####
........#####...#....
#######.####.#....##.
#.....#.####...####.#
#.###.#.####.##.#.#.#
#.###.#.#########.###
#.###.#.##....#....##
#.....#.#..#.##.####.
#######.##....#..#.##
"; // Includes a newline at the end
        assert_eq!(builder.to_string(), expected);
    }

    #[test]
    fn build() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.build("HELLO WORLD", &ECLevel::Q);
        println!("{}", builder.to_string());
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
