use bitvec::*;

/// Matrix is a 2-dimensional grid holding the QR modules.
pub struct Matrix {
    /// Size defines the width and height of the matrix.
    pub size: usize,

    /// The modules, dark is false.
    pub modules: BitVec,

    /// Functions, if set marks the bit as a function.
    pub functions: BitVec,
}

impl Matrix {
    /// Create a new matrix, modules initialized to false.
    pub fn new(size: usize) -> Matrix {
        Matrix {
            size: size,
            modules: bitvec![0; size * size],
            functions: bitvec![0; size * size],
        }
    }

    /// Map (x,y) coords to linear index.
    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.size);
        assert!(y < self.size);
        self.size * y + x
    }

    /// Returns true if the module at x,y is set.
    pub fn is_set(&self, x: usize, y: usize) -> bool {
        let i = self.index(x, y);
        self.modules[i]
    }

    /// Set a module.
    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        let i = self.index(x, y);
        self.modules.set(i, v);
    }

    /// Set square outline.
    pub fn set_square_outline(&mut self, x: usize, y: usize, w: usize, v: bool) {
        // Above and below
        for a in x..x + w {
            self.set(a, y, v);
            self.set(a, y + w - 1, v);
        }
        // Left and right
        for b in y + 1..y + w - 1 {
            self.set(x, b, v);
            self.set(x + w - 1, b, v);
        }
    }

    /// Returns true if the module at x,y is a function.
    pub fn is_fun(&self, x: usize, y: usize) -> bool {
        let i = self.index(x, y);
        self.functions[i]
    }

    /// Mark module as function.
    pub fn mark_fun(&mut self, x: usize, y: usize) {
        let i = self.index(x, y);
        self.functions.set(i, true);
    }

    /// Mark modules in rect as functions.
    pub fn mark_fun_square(&mut self, x: usize, y: usize, w: usize) {
        self.mark_fun_rect(x, y, x + w - 1, y + w - 1);
    }

    /// Mark modules in rect as functions.
    pub fn mark_fun_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for a in x0..x1 + 1 {
            for b in y0..y1 + 1 {
                self.mark_fun(a, b);
            }
        }
    }

    /// Return true if any module in a rect is marked as function
    pub fn any_fun_in_square(&self, x: usize, y: usize, w: usize) -> bool {
        for b in y..y + w {
            for a in x..x + w {
                if self.is_fun(a, b) {
                    return true;
                }
            }
        }
        false
    }
}

