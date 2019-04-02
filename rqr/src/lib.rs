
pub fn qr(x: &str) {
    // Data analysis to determine the most efficient mode
    //    numeric mode
    //    alphanumeric mode (no lowercase)
    //    UTF-8
    //    Kanji
    //
    // Data Encoding
    // 1. Choose error correction level
    //    L     recovers 7% of data
    //    M     recovers 15% of data
    //    Q     recovers 25% of data
    //    H     recovers 30% of data
    // 2. Determine smallest version of the data
    //    Called versions, 40 versions available
    //    See table for limits
    // 3. Add mode indicator
    // 4. Add character count indicator
    // 5. Encode using selected mode
    // 6. Break up into 8-bit codewords and pad if necessary
    //
    // Error correction coding
    //
    // Structure final message
    //
    // Module placement
    //
    // Data masking
    //
    // Format & version information

    println!("Got {}", x);
}

