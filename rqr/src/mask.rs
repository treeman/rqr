//use bitvec::*;
use crate::matrix::Matrix;
use bitvec::*;
use std::cmp;

pub fn evaluate(matrix: &Matrix) -> u16 {
    let e1 = evaluate_5_in_line(matrix);
    let e2 = evaluate_2x2(matrix);
    let e3 = evaluate_dl_pattern(matrix);
    let e4 = evaluate_bw(matrix);
    e1 + e2 + e3 + e4
}

fn evaluate_5_in_line(matrix: &Matrix) -> u16 {
    let mut res = 0;
    for i in 0..matrix.size {
        res += eval_5_col(matrix, i);
        res += eval_5_row(matrix, i);
    }
    res
}

fn eval_5_row(matrix: &Matrix, y: usize) -> u16 {
    let mut res = 0;
    let mut from = 0;
    let mut curr = matrix.is_set(0, y);
    for x in 1..matrix.size {
        if matrix.is_set(x, y) == curr {
            res += diff_5(from, x)
        } else {
            from = x;
            curr = !curr;
        }
    }
    res
}

fn eval_5_col(matrix: &Matrix, x: usize) -> u16 {
    let mut res = 0;
    let mut from = 0;
    let mut curr = matrix.is_set(x, 0);
    for y in 1..matrix.size {
        if matrix.is_set(x, y) == curr {
            res += diff_5(from, y)
        } else {
            from = y;
            curr = !curr;
        }
    }
    res
}

fn diff_5(from: usize, to: usize) -> u16 {
    let diff = to - from + 1;
    if diff == 5 {
        3
    } else if diff > 5 {
        1
    } else {
        0
    }
}

fn evaluate_2x2(matrix: &Matrix) -> u16 {
    let mut squares = 0;
    for x in 0..matrix.size - 1 {
        for y in 0..matrix.size - 1 {
            let vals = [
                matrix.is_set(x, y),
                matrix.is_set(x + 1, y),
                matrix.is_set(x, y + 1),
                matrix.is_set(x + 1, y + 1)
            ];
            let set_count = vals.iter().filter(|x| **x).count();
            if set_count == 0 || set_count == 4 {
                squares += 1;
            }
        }
    }
    squares * 3
}

fn evaluate_dl_pattern(matrix: &Matrix) -> u16 {
    let mut count = 0;
    for i in 0..matrix.size {
        if i != 13 { continue; }
        count += count_dl_row(matrix, i);
        count += count_dl_col(matrix, i);
    }
    count * 40
}

fn count_dl_row(matrix: &Matrix, y: usize) -> u16 {
    let mut row = BitVec::with_capacity(matrix.size);
    for x in 0..matrix.size {
        row.push(matrix.is_set(x, y));
    }
    count_dl_patterns(&row)
}

fn count_dl_col(matrix: &Matrix, x: usize) -> u16 {
    let mut col = BitVec::with_capacity(matrix.size);
    println!("x {}", x);
    for y in 0..matrix.size {
        col.push(matrix.is_set(x, y));
    }
    count_dl_patterns(&col)
}

fn count_dl_patterns(bv: &BitVec) -> u16 {
    // FIXME static allocation of representation.
    let bv1 = bitvec![1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0];
    let bv2 = bitvec![0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1];
    let mut res = 0;
    for w in bv.windows(11) {
        if w.iter().zip(bv1.iter()).all(|(x, y)| x == y) {
            res += 1;
        }
        if w.iter().zip(bv2.iter()).all(|(x, y)| x == y) {
            res += 1;
        }
    }
    res
}

fn evaluate_bw(matrix: &Matrix) -> u16 {
    let total = matrix.size * matrix.size;
    let dark = matrix.modules.iter().filter(|x| *x).count();
    let ratio = ((dark as f32) / (total as f32) * 100.0) as i16;
    let low_5 = ratio - ratio % 5;
    let high_5 = low_5 + 5;
    let a = 10 * (50 - low_5).abs() / 5;
    let b = 10 * (50 - high_5).abs() / 5;
    cmp::min(a, b) as u16
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::QrBuilder;
    use crate::version::Version;
    use crate::ec::ECLevel;

    #[test]
    fn masking() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.build_until_masking("HELLO WORLD", &ECLevel::Q);
        println!("{}", builder.to_string());
        assert_eq!(evaluate_5_in_line(&builder.matrix), 211);
        assert_eq!(evaluate_2x2(&builder.matrix), 135);
        assert_eq!(evaluate_dl_pattern(&builder.matrix), 40);
        assert_eq!(evaluate_bw(&builder.matrix), 10);
        assert_eq!(evaluate(&builder.matrix), 396);
        assert!(false);
    }
}

