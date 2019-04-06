use crate::data_encoding::Version;
use crate::block_info::*;
use bitvec::*;

/// Error correction level
#[derive(Debug, Copy, Clone)]
pub enum ECLevel {
    L, // Recovers 7% of data
    M, // Recovers 15% of data
    Q, // Recovers 25% of data
    H, // Recovers 30% of data
}

fn group_into_blocks(bv: &BitVec, v: &Version, ecl: &ECLevel) -> Vec<Vec<u8>> {
    let layout = group_block_count(v, ecl);
    let data = bv.as_slice();
    assert_eq!(data.len(), layout.iter().sum());

    let mut res = Vec::with_capacity(layout.len());
    let mut data_it = data.iter();
    for block in layout.iter() {
        let mut block_v:Vec<u8> = Vec::with_capacity(*block);
        for _ in 0..*block {
            block_v.push(*data_it.next().unwrap());
        }
        res.push(block_v);

        // Maybe there's a more idiomatic way to populate res using the take() method on data_it,
        // but I couldn't get past the borrow checker. I tried this:
        //res.push(data_it.take(*block).map(|x| *x as u8).collect());
    }
    res
}

fn generate_ec_codewords() -> Vec<u8> {
    // FIXME
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_tutorial_example() {
        let bv: BitVec = vec![
            0b01000011, 0b01010101, 0b01000110, 0b10000110, 0b01010111, 0b00100110,
            0b01010101, 0b11000010, 0b01110111, 0b00110010, 0b00000110, 0b00010010,
            0b00000110, 0b01100111, 0b00100110, 0b11110110, 0b11110110, 0b01000010,
            0b00000111, 0b01110110, 0b10000110, 0b11110010, 0b00000111, 0b00100110,
            0b01010110, 0b00010110, 0b11000110, 0b11000111, 0b10010010, 0b00000110,
            0b10110110, 0b11100110, 0b11110111, 0b01110111, 0b00110010, 0b00000111,
            0b01110110, 0b10000110, 0b01010111, 0b00100110, 0b01010010, 0b00000110,
            0b10000110, 0b10010111, 0b00110010, 0b00000111, 0b01000110, 0b11110111,
            0b01110110, 0b01010110, 0b11000010, 0b00000110, 0b10010111, 0b00110010,
            0b11100000, 0b11101100, 0b00010001, 0b11101100, 0b00010001, 0b11101100,
            0b00010001, 0b11101100].into();
        let expected = vec![
            vec![0b01000011, 0b01010101, 0b01000110, 0b10000110, 0b01010111,
                 0b00100110, 0b01010101, 0b11000010, 0b01110111, 0b00110010,
                 0b00000110, 0b00010010, 0b00000110, 0b01100111, 0b00100110],
            vec![0b11110110, 0b11110110, 0b01000010, 0b00000111, 0b01110110,
                 0b10000110, 0b11110010, 0b00000111, 0b00100110, 0b01010110,
                 0b00010110, 0b11000110, 0b11000111, 0b10010010, 0b00000110],
            vec![0b10110110, 0b11100110, 0b11110111, 0b01110111, 0b00110010,
                 0b00000111, 0b01110110, 0b10000110, 0b01010111, 0b00100110,
                 0b01010010, 0b00000110, 0b10000110, 0b10010111, 0b00110010,
                 0b00000111],
            vec![0b01000110, 0b11110111, 0b01110110, 0b01010110, 0b11000010,
                 0b00000110, 0b10010111, 0b00110010, 0b11100000, 0b11101100,
                 0b00010001, 0b11101100, 0b00010001, 0b11101100, 0b00010001,
                 0b11101100]
        ];
        assert_eq!(group_into_blocks(&bv, &Version::new(5), &ECLevel::Q),
                   expected);
    }

    #[test]
    fn ec_tutorial_example() {
        // FIXME
    }
}

