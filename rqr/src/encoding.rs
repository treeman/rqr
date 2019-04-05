//mod mode;
use crate::mode::Mode;
use bitvec::*;

/// Error correction level
#[derive(Debug)]
pub enum ECLevel {
    L, // Recovers 7% of data
    M, // Recovers 15% of data
    Q, // Recovers 25% of data
    H, // Recovers 30% of data
}


/// QR code version, defines the size
#[derive(Debug)]
pub struct Version(u8);

impl Version {
    pub fn new(v: u8) -> Version {
        assert!(v >= 1 && v <= 40);
        Version(v)
    }

    pub fn minimal(_mode: &Mode, _e: &ECLevel) -> Version {
        // TODO minimal version calculation
        Version(1)
    }
}

fn bitvec_mode(mode: &Mode) -> BitVec {
    match mode {
        Mode::Numeric(_) => bitvec![0, 0, 0, 1],
        Mode::Alphanumeric(_) => bitvec![0, 0, 1, 0],
        Mode::Byte(_) => bitvec![0, 1, 0, 0],
    }
}

fn required_len(mode: &Mode, version: &Version) -> usize {
    let v = version.0;
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
    let required = required_len(mode, v);
    let len = mode.len();

    let mut bv = BitVec::new();
    bv.reserve(required);
    for i in (0..required - 1).rev() {
        bv.push(len & (1 << i) != 0);
    }
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
    let numeric_bit_len = |num: u16| {
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
        let bit_len = numeric_bit_len(num);
        for i in (0..bit_len).rev() {
            bv.push(num & (1 << i) != 0);
        }
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
    // FIXME loop over each pair
    //for x in v.iter() {
        //let num: u16 = 
    //}
    //
    //let mut it = v.iter();
    ////while let 
    //loop {
        //if let Some(x) = 
    //}
    //for x in v.iter() {
        ////let mut num: u16
    //}
    v[..].into() // FIXME
}
fn encode_byte_data(v: &Vec<u8>) -> BitVec {
    // It's already in ISO 8859-1, or UTF-8
    v[..].into()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_data() {
        let numeric = Mode::Numeric(vec![0, 1, 2]);
        let byte = Mode::Byte(vec![140, 141, 142]);

        assert_eq!(bitvec_mode(&numeric), bitvec![0, 0, 0, 1]);

        assert_eq!(required_len(&numeric, &Version(1)), 10);
        assert_eq!(required_len(&byte, &Version(40)), 16);

        //// This is how we can build a bitvec
        //let bv: BitVec = vec![3, 9].into();
        //assert_eq!(bv.as_slice(), &[3, 9]);
        //let src: &[u8] = &[3, 9, 14];
        //let bv2: BitVec = src.into();
        //println!("bv2: {}", bv2);
        //assert_eq!(bv2.as_slice(), &[3, 9, 14]);

        assert_eq!(bitvec_char_count(&numeric, &Version(1)),
                   bitvec![0, 0, 0, 0, 0, 0, 0, 1, 1]);

        assert_eq!(encode_numeric_data(&vec![8, 6, 7, 5, 3, 0, 9]),
                   bitvec![1, 1, 0, 1, 1, 0, 0, 0, 1, 1, // 867
                           1, 0, 0, 0, 0, 1, 0, 0, 1, 0, // 530
                           1, 0, 0, 1]); // 9
    }
}


