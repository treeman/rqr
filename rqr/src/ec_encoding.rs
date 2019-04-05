
/// Error correction level
#[derive(Debug, Copy, Clone)]
pub enum ECLevel {
    L, // Recovers 7% of data
    M, // Recovers 15% of data
    Q, // Recovers 25% of data
    H, // Recovers 30% of data
}

