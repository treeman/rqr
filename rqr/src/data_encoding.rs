//mod mode;
use crate::mode::Mode;
use crate::ec::ECLevel;
use crate::version::Version;
use crate::block_info::*;

use bitvec::*;
use std::cmp;

pub fn encode(mode: &Mode, version: &Version, ecl: &ECLevel) -> BitVec {
    let total_capacity = total_bits(version, ecl);

    let mut bv = bitvec_mode(mode);
    bv.reserve(total_capacity);
    bv.append(&mut bitvec_char_count(mode, version));
    // FIXME here we can decide on the minimal version?
    bv.append(&mut bitvec_data(mode));
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

fn bitvec_mode(mode: &Mode) -> BitVec {
    match mode {
        Mode::Numeric(_) => bitvec![0, 0, 0, 1],
        Mode::Alphanumeric(_) => bitvec![0, 0, 1, 0],
        Mode::Byte(_) => bitvec![0, 1, 0, 0],
    }
}

fn char_count_len(mode: &Mode, version: &Version) -> usize {
    let v = version.v();
    if v <= 9 {
        match mode {
            Mode::Numeric(_) => 10,
            Mode::Alphanumeric(_) => 9,
            Mode::Byte(_) => 8,
        }
    } else if v <= 26 {
        match mode {
            Mode::Numeric(_) => 12,
            Mode::Alphanumeric(_) => 11,
            Mode::Byte(_) => 16,
        }
    } else if v <= 40 {
        match mode {
            Mode::Numeric(_) => 14,
            Mode::Alphanumeric(_) => 13,
            Mode::Byte(_) => 16,
        }
    } else {
        panic!("Malformed version {}", v);
    }
}

fn bitvec_char_count(mode: &Mode, v: &Version) -> BitVec {
    let required = char_count_len(mode, v);
    let char_count = mode.len();

    let mut bv = BitVec::new();
    append(&mut bv, char_count as u16, required);
    bv
}

fn bitvec_data(mode: &Mode) -> BitVec {
    match mode {
        Mode::Numeric(v) => encode_numeric_data(v),
        Mode::Alphanumeric(v) => encode_alphanumeric_data(v),
        Mode::Byte(v) => encode_byte_data(v),
    }
}

fn encode_numeric_data(v: &Vec<u8>) -> BitVec {
    // Encoding is done by grouping into groups of three
    // and converting that to binary.

    // If both first numbers are zero, convert it uses 4 bits.
    // If the first number in the group is zero it should use 7 bits.
    // Otherwise it should use 10 bits.
    // It's the minimal amount of bits that can all numbers of that length.
    let bit_len = |num: u16| {
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
        let num: u16 = s.parse().unwrap();
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
            // If there are two numbers, offset the first with * 45
            // as there are 45 possible characters, it fits into 11 bits.
            let num = 45 * (v[i] as u16) + (v[i + 1] as u16);
            append(&mut bv, num, 11);
        } else {
            // Otherwise 45 needs 6 bits.
            let num = v[i] as u16;
            append(&mut bv, num, 6);
        }
    }

    bv
}
fn encode_byte_data(v: &Vec<u8>) -> BitVec {
    // It's already in ISO 8859-1, or UTF-8
    v[..].into()
}

fn append(bv: &mut BitVec, v: u16, len: usize) {
    bv.extend((0..len).rev().map(|i| (v >> i) & 1 != 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_data() {
        let numeric = Mode::Numeric(vec![0, 1, 2]);
        let byte = Mode::Byte(vec![140, 141, 142]);

        let hello_alpha = Mode::new("HELLO WORLD");

        assert_eq!(bitvec_mode(&numeric), bitvec![0, 0, 0, 1]);

        assert_eq!(char_count_len(&numeric, &Version::new(1)), 10);
        assert_eq!(char_count_len(&byte, &Version::new(40)), 16);

        assert_eq!(bitvec_char_count(&numeric, &Version::new(1)),
                   bitvec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
        assert_eq!(bitvec_char_count(&hello_alpha, &Version::new(1)),
                   bitvec![0, 0, 0, 0, 0, 1, 0, 1, 1]);

        assert_eq!(encode_numeric_data(&vec![8, 6, 7, 5, 3, 0, 9]),
                   bitvec![1, 1, 0, 1, 1, 0, 0, 0, 1, 1, // 867
                           1, 0, 0, 0, 0, 1, 0, 0, 1, 0, // 530
                           1, 0, 0, 1]); // 9
        assert_eq!(encode_alphanumeric_data(&vec![17, 14]),
                   bitvec![0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1]);
        assert_eq!(encode_alphanumeric_data(&vec![45]),
                   bitvec![1, 0, 1, 1, 0, 1]);

        let hello_res: BitVec = vec![0b00100000, 0b01011011, 0b00001011, 0b01111000,
                                     0b11010001, 0b01110010, 0b11011100, 0b01001101,
                                     0b01000011, 0b01000000,
                                     // Three padding bytes
                                     0b11101100, 0b00010001, 0b11101100].into();
        assert_eq!(encode(&hello_alpha, &Version::new(1), &ECLevel::Q),
                   hello_res);
    }
}


