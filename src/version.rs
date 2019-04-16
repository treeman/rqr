//! Specifies the version of a QR code.

use crate::mode::Mode;
use crate::ec::ECLevel;

/// QR code version, defines the size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(pub usize);

impl Version {
    /// Create a new version, must be in the [1..40] range.
    pub fn new(v: usize) -> Version {
        assert!(v >= 1 && v <= 40);
        Version(v)
    }

    /// Calculate the minimal required version to hold the string
    /// in the given mode with the required error correction level.
    pub fn minimal(s: &str, mode: Mode, e: ECLevel) -> Option<Version> {
        let len = s.len();
        for v in 0..40 {
            let capacity = CAPACITIES[v][e as usize][mode as usize];
            if len <= capacity {
                return Some(Version(v + 1));
            }
        }
        None
    }

    /// Return the data capacity.
    pub fn capacity(&self, mode: Mode, e: ECLevel) -> usize {
        CAPACITIES[self.index()][e as usize][mode as usize]
    }

    /// Return the size of the QR code.
    pub fn size(&self) -> usize {
        (((self.index()) * 4) + 21)
    }

    /// Returns the required len of the char count bit representation.
    pub fn char_count_len(&self, mode: Mode) -> usize {
        if self.0 >= 1 && self.0 <= 9 {
            match mode {
                Mode::Numeric => 10,
                Mode::Alphanumeric => 9,
                Mode::Byte => 8,
            }
        } else if self.0 <= 26 {
            match mode {
                Mode::Numeric => 12,
                Mode::Alphanumeric => 11,
                Mode::Byte => 16,
            }
        } else if self.0 <= 40 {
            match mode {
                Mode::Numeric => 14,
                Mode::Alphanumeric => 13,
                Mode::Byte => 16,
            }
        } else {
            panic!("Malformed version {}", self.0);
        }
    }

    /// Returns true if this version requires extra version areas.
    pub fn extra_version_areas(&self) -> bool {
        self.0 >= 7
    }

    /// Returns the position of the dark module.
    pub fn dark_module_pos(&self) -> (usize, usize) {
        (8, 4 * self.0 + 9)
    }

    /// Return the version value - 1, suitable for indexing.
    pub fn index(&self) -> usize {
        (self.0 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(Version::new(32).size(), 145);

        assert_eq!(Version::new(1).char_count_len(Mode::Numeric), 10);
        assert_eq!(Version::new(40).char_count_len(Mode::Byte), 16);
    }

    #[test]
    fn minimal_version() {
        assert_eq!(Version::minimal("HELLO WORLD", Mode::Alphanumeric, ECLevel::Q),
                   Some(Version::new(1)));
        assert_eq!(Version::minimal("HELLO THERE WORLD", Mode::Alphanumeric, ECLevel::Q),
                   Some(Version::new(2)));

        // Test max possible capacity.
        let mut long = String::new();
        for _ in 0..2953 {
            long.push('X');
        }
        assert_eq!(Version::minimal(&long, Mode::Byte, ECLevel::L),
                   Some(Version::new(40)));
        // Go over largest capacity.
        long.push('X');
        assert_eq!(Version::minimal(&long, Mode::Byte, ECLevel::L),
                   None)
    }
}

// Capacities.
// Version x ECLevel x Mode (numeric, alphanumeric, byte)
static CAPACITIES: [[[usize; 3]; 4]; 40] = [
    [[41, 25, 17], [34, 20, 14], [27, 16, 11], [17, 10, 7]],
    [[77, 47, 32], [63, 38, 26], [48, 29, 20], [34, 20, 14]],
    [[127, 77, 53], [101, 61, 42], [77, 47, 32], [58, 35, 24]],
    [[187, 114, 78], [149, 90, 62], [111, 67, 46], [82, 50, 34]],
    [[255, 154, 106], [202, 122, 84], [144, 87, 60], [106, 64, 44]],
    [[322, 195, 134], [255, 154, 106], [178, 108, 74], [139, 84, 58]],
    [[370, 224, 154], [293, 178, 122], [207, 125, 86], [154, 93, 64]],
    [[461, 279, 192], [365, 221, 152], [259, 157, 108], [202, 122, 84]],
    [[552, 335, 230], [432, 262, 180], [312, 189, 130], [235, 143, 98]],
    [[652, 395, 271], [513, 311, 213], [364, 221, 151], [288, 174, 119]],
    [[772, 468, 321], [604, 366, 251], [427, 259, 177], [331, 200, 137]],
    [[883, 535, 367], [691, 419, 287], [489, 296, 203], [374, 227, 155]],
    [[1022, 619, 425], [796, 483, 331], [580, 352, 241], [427, 259, 177]],
    [[1101, 667, 458], [871, 528, 362], [621, 376, 258], [468, 283, 194]],
    [[1250, 758, 520], [991, 600, 412], [703, 426, 292], [530, 321, 220]],
    [[1408, 854, 586], [1082, 656, 450], [775, 470, 322], [602, 365, 250]],
    [[1548, 938, 644], [1212, 734, 504], [876, 531, 364], [674, 408, 280]],
    [[1725, 1046, 718], [1346, 816, 560], [948, 574, 394], [746, 452, 310]],
    [[1903, 1153, 792], [1500, 909, 624], [1063, 644, 442], [813, 493, 338]],
    [[2061, 1249, 858], [1600, 970, 666], [1159, 702, 482], [919, 557, 382]],
    [[2232, 1352, 929], [1708, 1035, 711], [1224, 742, 509], [969, 587, 403]],
    [[2409, 1460, 1003], [1872, 1134, 779], [1358, 823, 565], [1056, 640, 439]],
    [[2620, 1588, 1091], [2059, 1248, 857], [1468, 890, 611], [1108, 672, 461]],
    [[2812, 1704, 1171], [2188, 1326, 911], [1588, 963, 661], [1228, 744, 511]],
    [[3057, 1853, 1273], [2395, 1451, 997], [1718, 1041, 715], [1286, 779, 535]],
    [[3283, 1990, 1367], [2544, 1542, 1059], [1804, 1094, 751], [1425, 864, 593]],
    [[3517, 2132, 1465], [2701, 1637, 1125], [1933, 1172, 805], [1501, 910, 625]],
    [[3669, 2223, 1528], [2857, 1732, 1190], [2085, 1263, 868], [1581, 958, 658]],
    [[3909, 2369, 1628], [3035, 1839, 1264], [2181, 1322, 908], [1677, 1016, 698]],
    [[4158, 2520, 1732], [3289, 1994, 1370], [2358, 1429, 982], [1782, 1080, 742]],
    [[4417, 2677, 1840], [3486, 2113, 1452], [2473, 1499, 1030], [1897, 1150, 790]],
    [[4686, 2840, 1952], [3693, 2238, 1538], [2670, 1618, 1112], [2022, 1226, 842]],
    [[4965, 3009, 2068], [3909, 2369, 1628], [2805, 1700, 1168], [2157, 1307, 898]],
    [[5253, 3183, 2188], [4134, 2506, 1722], [2949, 1787, 1228], [2301, 1394, 958]],
    [[5529, 3351, 2303], [4343, 2632, 1809], [3081, 1867, 1283], [2361, 1431, 983]],
    [[5836, 3537, 2431], [4588, 2780, 1911], [3244, 1966, 1351], [2524, 1530, 1051]],
    [[6153, 3729, 2563], [4775, 2894, 1989], [3417, 2071, 1423], [2625, 1591, 1093]],
    [[6479, 3927, 2699], [5039, 3054, 2099], [3599, 2181, 1499], [2735, 1658, 1139]],
    [[6743, 4087, 2809], [5313, 3220, 2213], [3791, 2298, 1579], [2927, 1774, 1219]],
    [[7089, 4296, 2953], [5596, 3391, 2331], [3993, 2420, 1663], [3057, 1852, 1273]],
];
