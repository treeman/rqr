//! The matrix holds all modules in a QR code.
//!
//! They're organized in a 2-dimensional grid but held in
//! a vector internally.
//!
//! The implementation uses assertions heavily to ensure correctness.
//! If interfacing with the matrix directly take care not to violate
//! assumptions, like overwriting existing data.

use std::ops::Not;

/// The type of a module.
/// Differentiates the different types during construction,
/// a valid QR code should only hold function and data modules.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Module {
    /// An unknown module, it hasn't been assigned yet.
    Unknown,
    /// Reserved module.
    /// Version and format info uses this to reserve modules before masking.
    Reserved,
    /// Function module, contains QR code artifacts like finders and timing patterns.
    Function(bool),
    /// Data module. Contains both data and error codes.
    Data(bool),
}

impl Module {
    /// Is the module dark?
    /// Only makes sense for data or function modules.
    pub fn is_dark(&self) -> bool {
        match self {
            Module::Unknown => false,
            Module::Reserved => false,
            Module::Function(v) => *v,
            Module::Data(v) => *v,
        }
    }

    /// Is the module a function module?
    /// This includes reserved modules as well.
    pub fn is_fun(&self) -> bool {
        match self {
            Module::Unknown => false,
            Module::Data(_) => false,
            _ => true,
        }
    }

    /// Is the module a Data module?
    pub fn is_data(&self) -> bool {
        match self {
            Module::Data(_) => true,
            _ => false,
        }
    }
}

impl Not for Module {
    type Output = Module;
    fn not(self) -> Module {
        match self {
            Module::Unknown => Module::Unknown,
            Module::Reserved => Module::Reserved,
            Module::Function(v) => Module::Function(!v),
            Module::Data(v) => Module::Data(!v),
        }
    }
}

/// Matrix is a 2-dimensional grid holding the QR modules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix {
    /// Size defines the width and height of the matrix.
    pub size: usize,

    /// The modules.
    pub modules: Vec<Module>,
}

impl Matrix {
    /// Create a new matrix, modules initialized to Unknown.
    pub fn new(size: usize) -> Matrix {
        Matrix {
            size: size,
            modules: vec![Module::Unknown; size * size],
        }
    }

    /// Map (x,y) coords to linear index.
    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.size);
        assert!(y < self.size);
        self.size * y + x
    }

    /// Get module.
    pub fn get(&self, x: usize, y: usize) -> &Module {
        &self.modules[self.index(x, y)]
    }

    /// Get mutable module.
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Module {
        let i = self.index(x, y);
        &mut self.modules[i]
    }

    /// Returns true if the module at x,y is dark.
    pub fn is_dark(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_dark()
    }

    /// Return true if the module at x,y is a function module.
    pub fn is_fun(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_fun()
    }

    /// Return true if the module at x,y contains data.
    pub fn is_data(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_data()
    }

    /// Assign a module.
    pub fn set(&mut self, x: usize, y: usize, v: Module) {
        *self.get_mut(x, y) = v;
    }

    /// Assign a function module.
    /// Fails if the existing module is a Data module.
    pub fn set_fun(&mut self, x: usize, y: usize, v: bool) {
        let m = self.get_mut(x, y);
        assert!(!m.is_data());
        *m = Module::Function(v);
    }

    /// Assign a data module.
    /// Fails unless the existing module is Unknown.
    pub fn set_data(&mut self, x: usize, y: usize, v: bool) {
        let m = self.get_mut(x, y);
        assert!(*m == Module::Unknown);
        *m = Module::Data(v);
    }

    /// Flip a module.
    /// Fails unless it's a data module.
    pub fn flip(&mut self, x: usize, y: usize) {
        let m = self.get_mut(x, y);
        assert!(m.is_data());
        *m = !*m;
    }

    /// Set square outline.
    pub fn set_square_outline(&mut self, x: usize, y: usize, w: usize, v: Module) {
        // Above and below
        for a in x..(x + w) {
            self.set(a, y, v);
            self.set(a, y + w - 1, v);
        }
        // Left and right
        for b in (y + 1)..(y + w - 1) {
            self.set(x, b, v);
            self.set(x + w - 1, b, v);
        }
    }

    /// Set square.
    pub fn set_square(&mut self, x: usize, y: usize, w: usize, v: Module) {
        self.set_rect(x, y, x + w - 1, y + w - 1, v);
    }

    /// Set rect.
    pub fn set_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, v: Module) {
        for a in x0..(x1 + 1) {
            for b in y0..(y1 + 1) {
                self.set(a, b, v);
            }
        }
    }

    /// Return true if there's any module other than Unknown in square.
    pub fn any_in_square(&self, x: usize, y: usize, w: usize) -> bool {
        self.any_in_rect(x, y, x + w - 1, y + w - 1)
    }

    /// Return true if there's any module other than Unknown in rect.
    pub fn any_in_rect(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> bool {
        for a in x0..(x1 + 1) {
            for b in y0..(y1 + 1) {
                if *self.get(a, b) != Module::Unknown {
                    return true;
                }
            }
        }
        false
    }

    /// Return true if the matrix is complete, that's if it only contains
    /// Data or Function modules.
    pub fn complete(&self) -> bool {
        for m in self.modules.iter() {
            match m {
                Module::Unknown => return false,
                Module::Reserved => return false,
                _ => {},
            }
        }
        true
    }
}

