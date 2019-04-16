use crate::mode::Mode;
use crate::ec::ECLevel;
use crate::version::Version;
use crate::info;

use bitvec::*;
use std::cmp;

/// Encode string data to BitVec.
/// Does not include error correction codes, it only encodes the data portion.
pub fn encode(s: &str, version: Version, ecl: ECLevel) -> (Mode, BitVec) {
    let mode = Mode::from_str(s);
    let encoded = encode_with_mode(s, mode, version, ecl);
    (mode, encoded)
}

/// Encode string data to BitVec in a specific mode.
/// Does not include error correction codes, it only encodes the data portion.
pub fn encode_with_mode(s: &str, mode: Mode, version: Version, ecl: ECLevel) -> BitVec {
    let total_capacity = info::total_bits(version, ecl);

    // Encoding is mode, char count, data.
    let mut bv = mode.to_bitvec();
    bv.reserve(total_capacity);
    bv.append(&mut bitvec_char_count(s.len(), mode, version));
    bv.append(&mut bitvec_data(s, mode));
    assert!(bv.len() <= total_capacity);

    // Add up to 4 zero bits if we're below capacity.
    let zero_bits = cmp::min(total_capacity - bv.len(), 4);
    append(&mut bv, 0, zero_bits);
    assert!(bv.len() <= total_capacity);

    // If we're still below capacity add zero bits until we have full bytes.
    let zero_bits = (total_capacity - bv.len()) % 8;
    append(&mut bv, 0, zero_bits);
    assert!(bv.len() % 8 == 0);

    // Until we reach our capacity add pad bytes.
    for pad in [0xEC, 0x11].iter().cycle() {
        if bv.len() >= total_capacity {
            break;
        }
        append(&mut bv, *pad, 8);
    }
    assert_eq!(bv.len(), total_capacity);

    bv
}

/// Append data to bitvec of a certain len.
pub fn append(bv: &mut BitVec, v: u32, len: usize) {
    bv.extend((0..len).rev().map(|i| (v >> i) & 1 != 0));
}

fn bitvec_char_count(len: usize, mode: Mode, v: Version) -> BitVec {
    let mut bv = BitVec::new();
    append(&mut bv, len as u32, v.char_count_len(mode));
    bv
}

fn bitvec_data(s: &str, mode: Mode) -> BitVec {
    let bytes = string_to_bytes(s, mode);

    match mode {
        Mode::Numeric => encode_numeric_data(&bytes),
        Mode::Alphanumeric => encode_alphanumeric_data(&bytes),
        Mode::Byte => encode_byte_data(&bytes),
    }
}

fn encode_numeric_data(v: &Vec<u8>) -> BitVec {
    // Encoding is done by grouping into groups of three
    // and converting that to binary.

    // If both first numbers are zero, convert it uses 4 bits.
    // If the first number in the group is zero it should use 7 bits.
    // Otherwise it should use 10 bits.
    // It's the minimal amount of bits that can all numbers of that length.
    let bit_len = |num: u32| {
        if num > 99 {
            10
        } else if num > 9 {
            7
        } else {
            4
        }
    };

    let mut bv = BitVec::new();
    bv.reserve(v.len() * 8);

    let mut add = |s: &str| {
        let num: u32 = s.parse().unwrap();
        let len = bit_len(num);
        append(&mut bv, num, len);
    };

    let mut acc = String::new();
    for x in v.iter() {
        acc.push_str(x.to_string().as_str());
        if acc.len() == 3 {
            add(acc.as_str());
            acc.clear();
        }
    }
    if !acc.is_empty() {
        add(acc.as_str());
    }

    bv
}

fn encode_alphanumeric_data(v: &Vec<u8>) -> BitVec {
    let mut bv = BitVec::new();
    bv.reserve(v.len() * 8);

    // Encoding is done by grouping into groups of two.
    for i in (0..v.len()).step_by(2) {
        if i + 1 < v.len() {
            // If there are two numbers, offset the first with * 46
            // as there are 45 possible characters, it fits into 11 bits.
            let num = 45 * (v[i] as u32) + (v[i + 1] as u32);
            append(&mut bv, num, 11);
        } else {
            // Otherwise 45 needs 6 bits.
            let num = v[i] as u32;
            append(&mut bv, num, 6);
        }
    }

    bv
}
fn encode_byte_data(v: &Vec<u8>) -> BitVec {
    // It's already in ISO 8859-1, or UTF-8
    v[..].into()
}

// Converts string to byte representation.
// Numeric and alphanumeric are compacted more.
fn string_to_bytes(s: &str, mode: Mode) -> Vec<u8> {
    match mode {
        Mode::Numeric =>
            s.bytes().map(convert_numeric).collect(),
        Mode::Alphanumeric =>
            s.chars().map(convert_alphanumeric).collect(),
        Mode::Byte =>
            s.bytes().collect(),
    }
}

fn convert_numeric(b: u8) -> u8 {
    b - 48
}

fn convert_alphanumeric(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'I' => 18,
        'J' => 19,
        'K' => 20,
        'L' => 21,
        'M' => 22,
        'N' => 23,
        'O' => 24,
        'P' => 25,
        'Q' => 26,
        'R' => 27,
        'S' => 28,
        'T' => 29,
        'U' => 30,
        'V' => 31,
        'W' => 32,
        'X' => 33,
        'Y' => 34,
        'Z' => 35,
        ' ' => 36,
        '$' => 37,
        '%' => 38,
        '*' => 39,
        '+' => 40,
        '-' => 41,
        '.' => 42,
        '/' => 43,
        ':' => 44,
        _ => panic!("Unsupported alphanumeric '{}'", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_full() {
        let hello_res: BitVec = vec![0b00100000, 0b01011011, 0b00001011, 0b01111000,
                                     0b11010001, 0b01110010, 0b11011100, 0b01001101,
                                     0b01000011, 0b01000000,
                                     // Three padding bytes
                                     0b11101100, 0b00010001, 0b11101100].into();
        let (_, encoded) = encode("HELLO WORLD", Version::new(1), ECLevel::Q);
        assert_eq!(encoded, hello_res);
    }

    #[test]
    fn internal() {
        assert_eq!(bitvec_char_count(3, Mode::Numeric, Version::new(1)),
                   bitvec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
        assert_eq!(bitvec_char_count("HELLO WORLD".len(), Mode::Alphanumeric, Version::new(1)),
                   bitvec![0, 0, 0, 0, 0, 1, 0, 1, 1]);

        assert_eq!(encode_numeric_data(&vec![8, 6, 7, 5, 3, 0, 9]),
                   bitvec![1, 1, 0, 1, 1, 0, 0, 0, 1, 1, // 867
                           1, 0, 0, 0, 0, 1, 0, 0, 1, 0, // 530
                           1, 0, 0, 1]); // 9
        assert_eq!(encode_alphanumeric_data(&vec![17, 14]),
                   bitvec![0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1]);
        assert_eq!(encode_alphanumeric_data(&vec![45]),
                   bitvec![1, 0, 1, 1, 0, 1]);

        assert_eq!(string_to_bytes("0123456789", Mode::Numeric),
                   vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(string_to_bytes("ABCXYZ 0123456789$%*+-./:", Mode::Alphanumeric),
                   vec![10, 11, 12, 33, 34, 35, 36,
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                        37, 38, 39, 40, 41, 42, 43, 44]);
        assert_eq!(string_to_bytes("☃", Mode::Byte),
                   vec![0b11100010, 0b10011000, 0b10000011]);
    }
}


