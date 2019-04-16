use crate::version::Version;
use crate::info;

use bitvec::*;

/// Error correction level
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ECLevel {
    L = 0, // Recovers 7% of data
    M, // Recovers 15% of data
    Q, // Recovers 25% of data
    H, // Recovers 30% of data
}

impl ECLevel {
    /// Returns the bit encoding. It is not the same as the enum order.
    pub fn to_bitvec(&self) -> BitVec {
        match self {
            ECLevel::L => bitvec![0, 1],
            ECLevel::M => bitvec![0, 0],
            ECLevel::Q => bitvec![1, 1],
            ECLevel::H => bitvec![1, 0],
        }
    }
}

/// Add error correction codewords to data.
pub fn add(data: BitVec, v: Version, ecl: ECLevel) -> BitVec {
    let layout = info::group_block_count(v, ecl);
    assert_eq!(data.len() / 8, layout.iter().sum());

    let blocks = group_into_blocks(&data, &layout);
    let mut bytes: Vec<u8> = Vec::with_capacity(data.len() / 8);

    // First interleave all codewords in blocks.
    let layout_max = layout.iter().max().unwrap();
    for i in 0..*layout_max {
        for block in blocks.iter() {
            if i < block.len() {
                bytes.push(block[i]);
            }
        }
    }

    // Then interleave all ec codewords in blocks.
    let ec_count = info::block_ec_count(v, ecl);
    let ec_blocks: Vec<Vec<u8>> = blocks.iter()
        .map(|x| generate_ec_codewords(x.as_slice(), ec_count))
        .collect();
    for i in 0..ec_count {
        for ec in ec_blocks.iter() {
            bytes.push(ec[i]);
        }
    }

    let mut res: BitVec = bytes.into();

    // Add padding remainder bits.
    let remainder = REMAINDER_BITS[v.index()];
    res.resize(res.len() + remainder, false);
    assert_eq!(res.len(), data.len() + 8 * ec_count * layout.len() + remainder);

    res
}

fn generate_ec_codewords(msg: &[u8], ec_count: usize) -> Vec<u8> {
    let gen = GEN_POLYS[ec_count];
    assert_eq!(gen.len(), ec_count);

    // res[i] corresponds to the constant before x^i.
    let mut res: Vec<u8> = msg.into();
    // Extending the vector effectively multiplies all constants with ec_count.
    res.resize(res.len() + ec_count, 0);

    for i in 0..msg.len() {
        let lead = res[i] as usize;
        // Term is zero, nothing to do.
        if lead == 0 {
            continue;
        }

        // Use alpha notation for multiplications.
        let alpha = LOG[lead] as usize;
        // For all remaining terms, xor with the current result.
        for (x, y) in res[i + 1..].iter_mut().zip(gen.iter()) {
            // All 2^n in GF(256) are precalculated.
            *x ^= EXP[((*y as usize) + alpha) % 255];
        }
    }
    // Last ec_count elements is our result.
    let v: Vec<u8> = res[msg.len()..].into();
    assert_eq!(v.len(), ec_count);
    v
}


fn group_into_blocks(bv: &BitVec, layout: &Vec<usize>) -> Vec<Vec<u8>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    // Helper method to crate a BitVec from a string with '0' and '1'.
    // Discards any other characters, like newlines.
    pub fn bitvec_from_bin_str(s: &str) -> BitVec {
        s.chars()
        .filter(|x| *x == '1' || *x == '0')
        .map(|x| x == '1')
        .collect()
    }

    #[test]
    fn ec_tutorial_example() {
        let ec_count = info::block_ec_count(Version::new(1), ECLevel::M);
        assert_eq!(ec_count, 10);

        // HELLO WORLD as 1-M code
        let data: [u8; 16] = [0b00100000, 0b01011011, 0b00001011, 0b01111000, 0b11010001,
                              0b01110010, 0b11011100, 0b01001101, 0b01000011, 0b01000000,
                              0b11101100, 0b00010001, 0b11101100, 0b00010001, 0b11101100,
                              0b00010001];
        assert_eq!(generate_ec_codewords(&data, ec_count),
                   vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23]);
    }

    #[test]
    fn group_tutorial_example() {
        let layout = info::group_block_count(Version::new(5), ECLevel::Q);
        assert_eq!(layout, vec![15, 15, 16, 16]);

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
        assert_eq!(group_into_blocks(&bv, &layout),
                   expected);
    }

    #[test]
    fn interleave_tutorial_example() {
        let bv: BitVec = vec![
            // group 1 block 1
            0b01000011, 0b01010101, 0b01000110, 0b10000110, 0b01010111, 0b00100110,
            0b01010101, 0b11000010, 0b01110111, 0b00110010, 0b00000110, 0b00010010,
            0b00000110, 0b01100111, 0b00100110,
            // group 1 block 2
            0b11110110, 0b11110110, 0b01000010, 0b00000111, 0b01110110, 0b10000110,
            0b11110010, 0b00000111, 0b00100110, 0b01010110, 0b00010110, 0b11000110,
            0b11000111, 0b10010010, 0b00000110,
            // group 2 block 1
            0b10110110, 0b11100110, 0b11110111, 0b01110111, 0b00110010, 0b00000111,
            0b01110110, 0b10000110, 0b01010111, 0b00100110, 0b01010010, 0b00000110,
            0b10000110, 0b10010111, 0b00110010, 0b00000111,
            // group 2 block 2
            0b01000110, 0b11110111, 0b01110110, 0b01010110, 0b11000010, 0b00000110,
            // Codeword #55 is 0b11100000 but converted to 16 dec in the tutorial.
            // Using 16 as the representation works out in the end.
            //                   -> 0b11100000
            0b10010111, 0b00110010, 0b00010000, 0b11101100, 0b00010001, 0b11101100,
            0b00010001, 0b11101100, 0b00010001, 0b11101100].into();
        let expected = bitvec_from_bin_str(
             "010000111111011010110110010001100101010111110110111001101111
              011101000110010000101111011101110110100001100000011101110111
              010101100101011101110110001100101100001000100110100001100000
              011100000110010101011111001001110110100101111100001000000111
              100001100011001001110111001001100101011100010000001100100101
              011000100110111011000000011000010110010100100001000100010010
              110001100000011011101100000001101100011110000110000100010110
              011110010010100101111110110000100110000001100011001000010001
              000001111110110011010101010101111001010011101011110001111100
              110001110100100111110000101101100000101100010000010100101101
              001111001101010010101101011100111100101001001100000110001111
              011110110110100001011001001111110001011111000100101100111011
              110111111001110111110010001000011110010111001000111011100110
              101011111000100001100100110000101000100110100001101111000011
              111111110111010110000001111001101010110010011010110100011011
              110101010010011011110001000100001010000000100101011010100011
              011011001000001110100001101000111111000000100000011011110111
              10001100000010110010001001111000010110001101111011000000000");
        assert_eq!(add(bv, Version::new(5), ECLevel::Q).len(),
                   expected.len());
    }

    #[test]
    fn add_simple() {
        // For smaller versions data should simply be followed by ec data.
        let version = Version::new(1);
        let ecl = ECLevel::Q;

        let (_, mut data) = data::encode("HELLO WORLD", version, ecl);

        let ec_count = info::block_ec_count(version, ecl);
        let mut ec_data: BitVec = generate_ec_codewords(data.as_slice(), ec_count).into();
        let ec = add(data.clone(), version, ecl);

        assert_eq!(ec.len(), data.len() + ec_data.len());

        let mut res: BitVec = BitVec::new();
        res.append(&mut data);
        res.append(&mut ec_data);
        assert_eq!(ec, res);
    }
}

// How many additional remainder bits needs to be added
// after interleaving blocks and ec codes?
// Only depends on the version.
static REMAINDER_BITS: [usize; 40] = [
    0, 7, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0];


// Encode 2^x in GF(256) arithmetic.
static EXP: [u8; 256] = [
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180,
    117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212,
    181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222,
    161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187,
    107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13,
    26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51,
    102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154,
    41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126,
    252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165,
    87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162,
    89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244,
    245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216,
    173, 71, 142, 1];


// Encode the inverse of EXP.
static LOG: [u8; 256] = [
    255, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141,
    239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142,
    218, 240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114, 166,
    6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148, 206, 143,
    150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126, 110, 107, 58,
    40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172, 115, 243, 167,
    87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24, 227, 165, 153, 119,
    38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149, 188, 207, 205, 144,
    135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132, 60, 57, 83, 71, 109,
    65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12, 111, 246, 108, 161, 59,
    82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95, 176, 156, 169, 160, 81,
    11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173, 232, 116, 214, 244, 234, 168,
    80, 88, 175];


// Generator polynomials for the different grades.
// Just hardcode them, I can't be bothered to write the generator myself...
static GEN_POLYS: [&[u8]; 70] = [
    &[],
    &[0],
    &[25, 1],
    &[198, 199, 3],
    &[75, 249, 78, 6],
    &[113, 164, 166, 119, 10],
    &[166, 0, 134, 5, 176, 15],
    &[87, 229, 146, 149, 238, 102, 21],
    &[175, 238, 208, 249, 215, 252, 196, 28],
    &[95, 246, 137, 231, 235, 149, 11, 123, 36],
    &[251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
    &[220, 192, 91, 194, 172, 177, 209, 116, 227, 10, 55],
    &[102, 43, 98, 121, 187, 113, 198, 143, 131, 87, 157, 66],
    &[74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78],
    &[199, 249, 155, 48, 190, 124, 218, 137, 216, 87, 207, 59, 22, 91],
    &[8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105],
    &[120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120],
    &[43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136],
    &[215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98, 96, 153],
    &[67, 3, 105, 153, 52, 90, 83, 17, 150, 159, 44, 128, 153, 133, 252, 222, 138, 220, 171],
    &[17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212, 188, 190],
    &[240, 233, 104, 247, 181, 140, 67, 98, 85, 200, 210, 115, 148, 137, 230, 36, 122, 254, 148, 175, 210],
    &[210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219, 134, 160, 105, 165, 231],
    &[171, 102, 146, 91, 49, 103, 65, 17, 193, 150, 14, 25, 183, 248, 94, 164, 224, 192, 1, 78, 56,
      147, 253],
    &[229, 121, 135, 48, 211, 117, 251, 126, 159, 180, 169, 152, 192, 226, 228, 218, 111, 0, 117, 232, 87,
      96, 227, 21],
    &[231, 181, 156, 39, 170, 26, 12, 59, 15, 148, 201, 54, 66, 237, 208, 99, 167, 144, 182, 95, 243,
      129, 178, 252, 10, 32, 32, 32, 32, 32, 32, 45],
    &[173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142, 13, 102, 48,
      227, 153, 145, 10, 32, 32, 32, 32, 32, 32, 218, 70],
    &[79, 228, 8, 165, 227, 21, 180, 29, 9, 237, 70, 99, 45, 58, 138, 135, 73, 126, 172, 94, 216,
      193, 157, 26, 10, 32, 32, 32, 32, 32, 32, 17, 149, 96],
    &[168, 223, 200, 104, 224, 234, 108, 180, 110, 190, 195, 147, 205, 27, 232, 201, 21, 43, 245, 87, 42,
      195, 212, 119, 10, 32, 32, 32, 32, 32, 32, 242, 37, 9, 123],
    &[156, 45, 183, 29, 151, 219, 54, 96, 249, 24, 136, 5, 241, 175, 189, 28, 75, 234, 150, 148, 23,
      9, 202, 162, 10, 32, 32, 32, 32, 32, 32, 68, 250, 140, 24, 151],
    &[41, 173, 145, 152, 216, 31, 179, 182, 50, 48, 110, 86, 239, 96, 222, 125, 42, 173, 226, 193, 224,
      130, 156, 37, 10, 32, 32, 32, 32, 32, 32, 251, 216, 238, 40, 192, 180],
    &[20, 37, 252, 93, 63, 75, 225, 31, 115, 83, 113, 39, 44, 73, 122, 137, 118, 119, 144, 248, 248,
      55, 1, 225, 10, 32, 32, 32, 32, 32, 32, 105, 123, 183, 117, 187, 200, 210],
    &[10, 6, 106, 190, 249, 167, 4, 67, 209, 138, 138, 32, 242, 123, 89, 27, 120, 185, 80, 156, 38,
      69, 171, 60, 10, 32, 32, 32, 32, 32, 32, 28, 222, 80, 52, 254, 185, 220, 241],
    &[245, 231, 55, 24, 71, 78, 76, 81, 225, 212, 173, 37, 215, 46, 119, 229, 245, 167, 126, 72, 181,
      94, 165, 210, 10, 32, 32, 32, 32, 32, 32, 98, 125, 159, 184, 169, 232, 185, 231, 18],
    &[111, 77, 146, 94, 26, 21, 108, 19, 105, 94, 113, 193, 86, 140, 163, 125, 58, 158, 229, 239, 218,
      103, 56, 70, 10, 32, 32, 32, 32, 32, 32, 114, 61, 183, 129, 167, 13, 98, 62, 129, 51],
    &[7, 94, 143, 81, 247, 127, 202, 202, 194, 125, 146, 29, 138, 162, 153, 65, 105, 122, 116, 238, 26,
      36, 216, 112, 10, 32, 32, 32, 32, 32, 32, 125, 228, 15, 49, 8, 162, 30, 126, 111, 58, 85],
    &[200, 183, 98, 16, 172, 31, 246, 234, 60, 152, 115, 0, 167, 152, 113, 248, 238, 107, 18, 63, 218,
      37, 87, 210, 10, 32, 32, 32, 32, 32, 32, 105, 177, 120, 74, 121, 196, 117, 251, 113, 233, 30, 120],
    &[154, 75, 141, 180, 61, 165, 104, 232, 46, 227, 96, 178, 92, 135, 57, 162, 120, 194, 212, 174, 252,
      183, 42, 35, 10, 32, 32, 32, 32, 32, 32, 157, 111, 23, 133, 100, 8, 105, 37, 192, 189, 159, 19, 156],
    &[159, 34, 38, 228, 230, 59, 243, 95, 49, 218, 176, 164, 20, 65, 45, 111, 39, 81, 49, 118, 113,
      222, 193, 250, 10, 32, 32, 32, 32, 32, 32, 242, 168, 217, 41, 164, 247, 177, 30, 238, 18, 120, 153,
      60, 193],
    &[81, 216, 174, 47, 200, 150, 59, 156, 89, 143, 89, 166, 183, 170, 152, 21, 165, 177, 113, 132, 234,
      5, 154, 68, 10, 32, 32, 32, 32, 32, 32, 124, 175, 196, 157, 249, 233, 83, 24, 153, 241, 126, 36, 116,
      19, 231],
    &[59, 116, 79, 161, 252, 98, 128, 205, 128, 161, 247, 57, 163, 56, 235, 106, 53, 26, 187, 174, 226,
      104, 170, 7, 10, 32, 32, 32, 32, 32, 32, 175, 35, 181, 114, 88, 41, 47, 163, 125, 134, 72, 20, 232,
      53, 35, 15],
    &[132, 167, 52, 139, 184, 223, 149, 92, 250, 18, 83, 33, 127, 109, 194, 7, 211, 242, 109, 66, 86,
      169, 87, 96, 10, 32, 32, 32, 32, 32, 32, 187, 159, 114, 172, 118, 208, 183, 200, 82, 179, 38,
      39, 34, 242, 142, 147, 55],
    &[250, 103, 221, 230, 25, 18, 137, 231, 0, 3, 58, 242, 221, 191, 110, 84, 230, 8, 188, 106, 96,
      147, 15, 131, 10, 32, 32, 32, 32, 32, 32, 139, 34, 101, 223, 39, 101, 213, 199, 237, 254, 201,
      123, 171, 162, 194, 117, 50, 96],
    &[96, 67, 3, 245, 217, 215, 33, 65, 240, 109, 144, 63, 21, 131, 38, 101, 153, 128, 55, 31, 237,
      3, 94, 160, 10, 32, 32, 32, 32, 32, 32, 20, 87, 77, 56, 191, 123, 207, 75, 82, 0, 122,
      132, 101, 145, 215, 15, 121, 192, 138],
    &[190, 7, 61, 121, 71, 246, 69, 55, 168, 188, 89, 243, 191, 25, 72, 123, 9, 145, 14, 247, 1,
      238, 44, 78, 10, 32, 32, 32, 32, 32, 32, 143, 62, 224, 126, 118, 114, 68, 163, 52, 194, 217,
      147, 204, 169, 37, 130, 113, 102, 73, 181],
    &[6, 172, 72, 250, 18, 171, 171, 162, 229, 187, 239, 4, 187, 11, 37, 228, 102, 72, 102, 22, 33,
      73, 95, 99, 10, 32, 32, 32, 32, 32, 32, 132, 1, 15, 89, 4, 112, 130, 95, 211, 235, 227,
      58, 35, 88, 132, 23, 44, 165, 54, 187, 225],
    &[112, 94, 88, 112, 253, 224, 202, 115, 187, 99, 89, 5, 54, 113, 129, 44, 58, 16, 135, 216, 169,
      211, 36, 1, 10, 32, 32, 32, 32, 32, 32, 4, 96, 60, 241, 73, 104, 234, 8, 249, 245, 119,
      174, 52, 25, 157, 224, 43, 202, 223, 19, 82, 15],
    &[76, 164, 229, 92, 79, 168, 219, 110, 104, 21, 220, 74, 19, 199, 195, 100, 93, 191, 43, 213, 72,
      56, 138, 161, 10, 32, 32, 32, 32, 32, 32, 125, 187, 119, 250, 189, 137, 190, 76, 126, 247, 93,
      30, 132, 6, 58, 213, 208, 165, 224, 152, 133, 91, 61],
    &[228, 25, 196, 130, 211, 146, 60, 24, 251, 90, 39, 102, 240, 61, 178, 63, 46, 123, 115, 18, 221,
      111, 135, 160, 10, 32, 32, 32, 32, 32, 32, 182, 205, 107, 206, 95, 150, 120, 184, 91, 21, 247,
      156, 140, 238, 191, 11, 94, 227, 84, 50, 163, 39, 34, 108],
    &[172, 121, 1, 41, 193, 222, 237, 64, 109, 181, 52, 120, 212, 226, 239, 245, 208, 20, 246, 34, 225,
      204, 134, 101, 10, 32, 32, 32, 32, 32, 32, 125, 206, 69, 138, 250, 0, 77, 58, 143, 185, 220,
      254, 210, 190, 112, 88, 91, 57, 90, 109, 5, 13, 181, 25, 10, 32, 32, 32, 32, 32, 32, 156],
    &[232, 125, 157, 161, 164, 9, 118, 46, 209, 99, 203, 193, 35, 3, 209, 111, 195, 242, 203, 225, 46,
      13, 32, 160, 10, 32, 32, 32, 32, 32, 32, 126, 209, 130, 160, 242, 215, 242, 75, 77, 42, 189,
      32, 113, 65, 124, 69, 228, 114, 235, 175, 124, 170, 215, 232, 10, 32, 32, 32, 32, 32, 32, 133, 205],
    &[213, 166, 142, 43, 10, 216, 141, 163, 172, 180, 102, 70, 89, 62, 222, 62, 42, 210, 151, 163, 218,
      70, 77, 39, 10, 32, 32, 32, 32, 32, 32, 166, 191, 114, 202, 245, 188, 183, 221, 75, 212, 27,
      237, 127, 204, 235, 62, 190, 232, 18, 46, 171, 15, 98, 247, 10, 32, 32, 32, 32, 32, 32, 66, 163, 0],
    &[116, 50, 86, 186, 50, 220, 251, 89, 192, 46, 86, 127, 124, 19, 184, 233, 151, 215, 22, 14, 59,
      145, 37, 242, 10, 32, 32, 32, 32, 32, 32, 203, 134, 254, 89, 190, 94, 59, 65, 124, 113, 100,
      233, 235, 121, 22, 76, 86, 97, 39, 242, 200, 220, 101, 33, 10, 32, 32, 32, 32, 32, 32, 239, 254,
      116, 51],
    &[122, 214, 231, 136, 199, 11, 6, 205, 124, 72, 213, 117, 187, 60, 147, 201, 73, 75, 33, 146, 171,
      247, 118, 208, 10, 32, 32, 32, 32, 32, 32, 157, 177, 203, 235, 83, 45, 226, 202, 229, 168, 7,
      57, 237, 235, 200, 124, 106, 254, 165, 14, 147, 0, 57, 42, 10, 32, 32, 32, 32, 32, 32, 31, 178, 213,
      173, 103],
    &[183, 26, 201, 87, 210, 221, 113, 21, 46, 65, 45, 50, 238, 184, 249, 225, 102, 58, 209, 218, 109,
      165, 26, 95, 10, 32, 32, 32, 32, 32, 32, 184, 192, 52, 245, 35, 254, 238, 175, 172, 79, 123,
      25, 122, 43, 120, 108, 215, 80, 128, 201, 235, 8, 153, 59, 10, 32, 32, 32, 32, 32, 32, 101, 31, 198,
      76, 31, 156],
    &[38, 197, 123, 167, 16, 87, 178, 238, 227, 97, 148, 247, 26, 90, 228, 182, 236, 197, 47, 249, 36,
      213, 54, 113, 10, 32, 32, 32, 32, 32, 32, 181, 74, 177, 204, 155, 61, 47, 42, 0, 132, 144,
      251, 200, 38, 38, 138, 54, 44, 64, 19, 22, 206, 16, 10, 10, 32, 32, 32, 32, 32, 32, 228,
      211, 161, 171, 44, 194, 210],
    &[106, 120, 107, 157, 164, 216, 112, 116, 2, 91, 248, 163, 36, 201, 202, 229, 6, 144, 254, 155, 135,
      208, 170, 209, 10, 32, 32, 32, 32, 32, 32, 12, 139, 127, 142, 182, 249, 177, 174, 190, 28, 10,
      85, 239, 184, 101, 124, 152, 206, 96, 23, 163, 61, 27, 196, 10, 32, 32, 32, 32, 32, 32, 247,
      151, 154, 202, 207, 20, 61, 10],
    &[58, 140, 237, 93, 106, 61, 193, 2, 87, 73, 194, 215, 159, 163, 10, 155, 5, 121, 153, 59, 248,
      4, 117, 22, 10, 32, 32, 32, 32, 32, 32, 60, 177, 144, 44, 72, 228, 62, 1, 19, 170, 113,
      158, 25, 175, 199, 139, 90, 1, 210, 7, 119, 154, 89, 159, 10, 32, 32, 32, 32, 32, 32, 130,
      122, 46, 147, 190, 135, 94, 68, 66],
    &[82, 116, 26, 247, 66, 27, 62, 107, 252, 182, 200, 185, 235, 55, 251, 242, 210, 144, 154, 237, 176,
      141, 192, 248, 10, 32, 32, 32, 32, 32, 32, 152, 249, 206, 85, 253, 142, 65, 165, 125, 23, 24,
      30, 122, 240, 214, 6, 129, 218, 29, 145, 127, 134, 206, 245, 10, 32, 32, 32, 32, 32, 32, 117,
      29, 41, 63, 159, 142, 233, 125, 148, 123],
    &[57, 115, 232, 11, 195, 217, 3, 206, 77, 67, 29, 166, 180, 106, 118, 203, 17, 69, 152, 213, 74,
      44, 49, 43, 10, 32, 32, 32, 32, 32, 32, 98, 61, 253, 122, 14, 43, 209, 143, 9, 104, 107,
      171, 224, 57, 254, 251, 226, 232, 221, 194, 240, 117, 161, 82, 10, 32, 32, 32, 32, 32, 32, 178,
      246, 178, 33, 50, 86, 215, 239, 180, 180, 181],
    &[107, 140, 26, 12, 9, 141, 243, 197, 226, 197, 219, 45, 211, 101, 219, 120, 28, 181, 127, 6, 100,
      247, 2, 205, 10, 32, 32, 32, 32, 32, 32, 198, 57, 115, 219, 101, 109, 160, 82, 37, 38, 238,
      49, 160, 209, 121, 86, 11, 124, 30, 181, 84, 25, 194, 87, 10, 32, 32, 32, 32, 32, 32, 65,
      102, 190, 220, 70, 27, 209, 16, 89, 7, 33, 240],
    &[161, 244, 105, 115, 64, 9, 221, 236, 16, 145, 148, 34, 144, 186, 13, 20, 254, 246, 38, 35, 202,
      72, 4, 212, 10, 32, 32, 32, 32, 32, 32, 159, 211, 165, 135, 252, 250, 25, 87, 30, 120, 226,
      234, 92, 199, 72, 7, 155, 218, 231, 44, 125, 178, 156, 174, 10, 32, 32, 32, 32, 32, 32, 124,
      43, 100, 31, 56, 101, 204, 64, 175, 225, 169, 146, 45],
    &[65, 202, 113, 98, 71, 223, 248, 118, 214, 94, 0, 122, 37, 23, 2, 228, 58, 121, 7, 105, 135,
      78, 243, 118, 10, 32, 32, 32, 32, 32, 32, 70, 76, 223, 89, 72, 50, 70, 111, 194, 17, 212,
      126, 181, 35, 221, 117, 235, 11, 229, 149, 147, 123, 213, 40, 10, 32, 32, 32, 32, 32, 32, 115,
      6, 200, 100, 26, 246, 182, 218, 127, 215, 36, 186, 110, 106],
    &[30, 71, 36, 71, 19, 195, 172, 110, 61, 2, 169, 194, 90, 136, 59, 182, 231, 145, 102, 39, 170,
      231, 214, 67, 10, 32, 32, 32, 32, 32, 32, 196, 207, 53, 112, 246, 90, 90, 121, 183, 146, 74, 77,
      38, 89, 22, 231, 55, 56, 242, 112, 217, 110, 123, 62, 10, 32, 32, 32, 32, 32, 32, 201, 217, 128,
      165, 60, 181, 37, 161, 246, 132, 246, 18, 115, 136, 168],
    &[45, 51, 175, 9, 7, 158, 159, 49, 68, 119, 92, 123, 177, 204, 187, 254, 200, 78, 141, 149, 119,
      26, 127, 53, 10, 32, 32, 32, 32, 32, 32, 160, 93, 199, 212, 29, 24, 145, 156, 208, 150, 218,
      209, 4, 216, 91, 47, 184, 146, 47, 140, 195, 195, 125, 242, 10, 32, 32, 32, 32, 32, 32, 238,
      63, 99, 108, 140, 230, 242, 31, 204, 11, 178, 243, 217, 156, 213, 231],
    &[137, 158, 247, 240, 37, 238, 214, 128, 99, 218, 46, 138, 198, 128, 92, 219, 109, 139, 166, 25, 66,
      67, 14, 58, 10, 32, 32, 32, 32, 32, 32, 238, 149, 177, 195, 221, 154, 171, 48, 80, 12, 59,
      190, 228, 19, 55, 208, 92, 112, 229, 37, 60, 10, 47, 81, 10, 32, 32, 32, 32, 32, 32, 0,
      192, 37, 171, 175, 147, 128, 73, 166, 61, 149, 12, 24, 95, 70, 113, 40],
    &[5, 118, 222, 180, 136, 136, 162, 51, 46, 117, 13, 215, 81, 17, 139, 247, 197, 171, 95, 173, 65,
      137, 178, 68, 10, 32, 32, 32, 32, 32, 32, 111, 95, 101, 41, 72, 214, 169, 197, 95, 7, 44,
      154, 77, 111, 236, 40, 121, 143, 63, 87, 80, 253, 240, 126, 10, 32, 32, 32, 32, 32, 32, 217,
      77, 34, 232, 106, 50, 168, 82, 76, 146, 67, 106, 171, 25, 132, 93, 45, 105],
    &[191, 172, 113, 86, 7, 166, 246, 185, 155, 250, 98, 113, 89, 86, 214, 225, 156, 190, 58, 33, 144,
      67, 179, 163, 10, 32, 32, 32, 32, 32, 32, 52, 154, 233, 151, 104, 251, 160, 126, 175, 208, 225,
      70, 227, 146, 4, 152, 139, 103, 25, 107, 61, 204, 159, 250, 10, 32, 32, 32, 32, 32, 32, 193,
      225, 105, 160, 98, 167, 2, 53, 16, 242, 83, 210, 196, 103, 248, 86, 211, 41, 171],
    &[247, 159, 223, 33, 224, 93, 77, 70, 90, 160, 32, 254, 43, 150, 84, 101, 190, 205, 133, 52, 60,
      202, 165, 220, 10, 32, 32, 32, 32, 32, 32, 203, 151, 93, 84, 15, 84, 253, 173, 160, 89, 227,
      52, 199, 97, 95, 231, 52, 177, 41, 125, 137, 241, 166, 225, 10, 32, 32, 32, 32, 32, 32, 118,
      2, 54, 32, 82, 215, 175, 198, 43, 238, 235, 27, 101, 184, 127, 3, 5, 8, 163, 238],
    &[105, 73, 68, 1, 29, 168, 117, 14, 88, 208, 55, 46, 42, 217, 6, 84, 179, 97, 6, 240, 192,
      231, 158, 64, 10, 32, 32, 32, 32, 32, 32, 118, 160, 203, 57, 61, 108, 199, 124, 65, 187, 221,
      167, 39, 182, 159, 180, 244, 203, 228, 254, 13, 175, 61, 90, 10, 32, 32, 32, 32, 32, 32, 206,
      40, 199, 94, 67, 57, 81, 229, 46, 123, 89, 37, 31, 202, 66, 250, 35, 170, 243, 88, 51]];


