use crate::matrix::Matrix;

use bitvec::*;
use std::cmp;
use lazy_static::lazy_static;

// TODO change mask from usize to enum

/// Evaluates masks.
/// Returns the mask with the lowest score and a matrix with the mask applied.
pub fn mask(matrix: &Matrix) -> (usize, Matrix) {
    let mut min_score = u16::max_value();
    let mut res = None;
    for mask in 0..8 {
        let masked = apply_mask(mask, matrix);
        let score = evaluate(&masked);
        if score < min_score {
            min_score = score;
            res = Some((mask, masked));
        }
    }
    res.unwrap()
}

/// Apply a mask of a specific type to a matrix.
pub fn apply_mask(mask: usize, matrix: &Matrix) -> Matrix {
    apply_mask_fun(mask_fun(mask), matrix)
}

/// Evaluate the mask score of a matrix.
pub fn evaluate(matrix: &Matrix) -> u16 {
    // It might be possible to combine these in the implementation to make it
    // more concice, but this is easier to understand, test and debug.
    let e1 = evaluate_5_in_line(matrix);
    let e2 = evaluate_2x2(matrix);
    let e3 = evaluate_dl_pattern(matrix);
    let e4 = evaluate_bw(matrix);
    e1 + e2 + e3 + e4
}

// 5 in a row/col should give a score of 3, each extra gives a score of 1.
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
    let mut curr = matrix.is_dark(0, y);
    for x in 1..matrix.size {
        if matrix.is_dark(x, y) == curr {
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
    let mut curr = matrix.is_dark(x, 0);
    for y in 1..matrix.size {
        if matrix.is_dark(x, y) == curr {
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

// Each 2x2 square of the same color gives a score of 3.
fn evaluate_2x2(matrix: &Matrix) -> u16 {
    let mut squares = 0;
    for x in 0..matrix.size - 1 {
        for y in 0..matrix.size - 1 {
            let square = [
                matrix.is_dark(x, y),
                matrix.is_dark(x + 1, y),
                matrix.is_dark(x, y + 1),
                matrix.is_dark(x + 1, y + 1)
            ];
            let set_count = square.iter().filter(|x| **x).count();
            if set_count == 0 || set_count == 4 {
                squares += 1;
            }
        }
    }
    squares * 3
}

// Each dark/light pattern found gives a score of 40.
fn evaluate_dl_pattern(matrix: &Matrix) -> u16 {
    let mut count = 0;
    for i in 0..matrix.size {
        count += count_dl_row(matrix, i);
        count += count_dl_col(matrix, i);
    }
    count * 40
}

fn count_dl_row(matrix: &Matrix, y: usize) -> u16 {
    let mut row = BitVec::with_capacity(matrix.size);
    for x in 0..matrix.size {
        row.push(!matrix.is_dark(x, y));
    }
    count_dl_patterns(&row)
}

fn count_dl_col(matrix: &Matrix, x: usize) -> u16 {
    let mut col = BitVec::with_capacity(matrix.size);
    for y in 0..matrix.size {
        col.push(!matrix.is_dark(x, y));
    }
    count_dl_patterns(&col)
}

lazy_static! {
    // Dark/light patterns we should detect.
    // BitVec can't be initialized in lazy_static so we'll use a standard Vec.
    // Convert to bool once here to make later comparisons simpler.
    // 0 is dark
    static ref DLP1: Vec<bool> = [1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0]
        .iter().map(|x| *x == 1).collect();
    static ref DLP2: Vec<bool> = [0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1]
        .iter().map(|x| *x == 1).collect();
}

fn count_dl_patterns(bv: &BitVec) -> u16 {
    let mut res = 0;
    // Each window is an iterator over 11 elements which we can
    // compare the patterns we search for against.
    for w in bv.windows(11) {
        if w.iter().zip(DLP1.iter()).all(|(x, y)| x == *y) {
            res += 1;
        }
        if w.iter().zip(DLP2.iter()).all(|(x, y)| x == *y) {
            res += 1;
        }
    }
    res
}

// Calculates a score depending on the light/dark ratio.
fn evaluate_bw(matrix: &Matrix) -> u16 {
    let total = matrix.size * matrix.size;
    let dark = matrix.modules.iter().filter(|x| x.is_dark()).count();
    let ratio = ((dark as f32) / (total as f32) * 100.0) as i16;
    let low_5 = ratio - ratio % 5;
    let high_5 = low_5 + 5;
    let a = 10 * (50 - low_5).abs() / 5;
    let b = 10 * (50 - high_5).abs() / 5;
    cmp::min(a, b) as u16
}

fn mask_fun(mask: usize) -> Box<Fn(usize, usize) -> bool> {
    match mask {
        0 => Box::new(move |x, y| (x + y) % 2 == 0),
        1 => Box::new(move |_, y| y % 2 ==0),
        2 => Box::new(move |x, _| x % 3 == 0),
        3 => Box::new(move |x, y| (x + y) % 3 == 0),
        4 => Box::new(move |x, y| ((y / 2) + (x / 3)) % 2 == 0),
        5 => Box::new(move |x, y| (x * y) % 2 + (x * y) % 3 == 0),
        6 => Box::new(move |x, y| ((x * y) % 2 + (x * y) % 3) % 2 == 0),
        7 => Box::new(move |x, y| ((x + y) % 2 + (x * y) % 3) % 2 == 0),
        _ => panic!("Unsupported mask: {}", mask),
    }
}

fn apply_mask_fun(f: Box<Fn(usize, usize) -> bool>, matrix: &Matrix) -> Matrix {
    let mut res = matrix.clone();
    for y in 0..res.size {
        for x in 0..res.size {
            if matrix.is_data(x, y) && f(x, y) {
                res.flip(x, y);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::QrBuilder;
    use crate::version::Version;
    use crate::ec::ECLevel;
    use crate::render;

    //#[test]
    //fn masking() {
        //let mut builder = QrBuilder::new(&Version::new(1));
        //builder.build_until_masking("HELLO WORLD", &ECLevel::Q);
        //let (best_mask, best_matrix) = mask(&builder.matrix);
        //assert_eq!(best_mask, 6);
        //println!("{}", render::to_string(&best_matrix));

        //let output =
//"#######.###.#.#######
//#.....#.#.##..#.....#
//#.###.#.#.#...#.###.#
//#.###.#.#.....#.###.#
//#.###.#.#.#.#.#.###.#
//#.....#.#.##..#.....#
//#######.#.#.#.#######
//........#.#..........
//#########.##.########
//.#......####....#...#
//##.#.##.###.##..#####
//.#..#..##.#..###..###
//..#...#....#...#.....
//........####.##.#.###
//#######.#..##..##....
//#.....#.##.##.##.#...
//#.###.#.#.#.##.###...
//#.###.#.##...###.#.##
//#.###.#.#.####.####..
//#.....#.#..##...##..#
//#######.#.#.#######.#
//"; // Includes a newline at the end
        //assert_eq!(render::to_string(&best_matrix), output);
    //}

    #[test]
    fn mask_evaluation() {
        let mut builder = QrBuilder::new(&Version::new(1));
        builder.add_fun_patterns();
        builder.add_data("HELLO WORLD", &ECLevel::Q);
        println!("{}", builder.to_string());
        assert_eq!(evaluate_5_in_line(&builder.matrix), 211);
        assert_eq!(evaluate_2x2(&builder.matrix), 135);
        assert_eq!(evaluate_dl_pattern(&builder.matrix), 80);
        assert_eq!(evaluate_bw(&builder.matrix), 10);
        assert_eq!(evaluate(&builder.matrix), 436);
    }
}

