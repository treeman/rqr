//use bitvec::*;

//pub fn evaluate(modules: &BitVec) -> u16 {
    //// TODO
    //// The first rule gives the QR code a penalty for each group of five or more
    //// same-colored modules in a row (or column).
    //// The second rule gives the QR code a penalty for each 2x2 area of
    //// same-colored modules in the matrix.
    //// The third rule gives the QR code a large penalty if there are patterns
    //// that look similar to the finder patterns.
    //// The fourth rule gives the QR code a penalty if more than half of the modules
    //// are dark or light, with a larger penalty for a larger difference.
    //let e1 = evaluate_1(modules);
    //println!("e1: {}", e1);
    //0
//}

//fn evaluate_1(modules: &BitVec) -> u16 {
    //let mut res = 0;
    //res
//}

//pub fn mask(modules: &BitVec) -> BitVec {
    //// TODO
    //modules.clone()
//}


//#[cfg(test)]
//mod tests {
    //use super::*;
    //use crate::builder::QrBuilder;
    //use crate::version::Version;
    //use crate::ec::ECLevel;

    //#[test]
    //fn masking() {
        //let mut builder = QrBuilder::new(&Version::new(1));
        //builder.build_until_masking("HELLO WORLD", &ECLevel::Q);
        //println!("{}", builder.to_string());
        //let e = evaluate(&builder.modules);
        //println!("total: {}", e);
        //assert!(false);
    //}
//}

