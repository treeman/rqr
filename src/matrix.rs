use std::ops::Not;

// false is dark
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Module {
    Unknown,
    //Finder(bool),
    //Separator(bool),
    //Alignment(bool),
    //Timing(bool),
    //Dark,
    //Reserved,
    //Format(bool),
    //Version(bool),
    Reserved,
    Function(bool),
    Data(bool),
}

impl Module {
    pub fn is_dark(&self) -> bool {
        match self {
            Module::Unknown => false,
            Module::Reserved => false,
            Module::Function(v) => !*v,
            Module::Data(v) => !*v,
        }
    }

    pub fn is_fun(&self) -> bool {
        match self {
            Module::Unknown => false,
            Module::Data(_) => false,
            _ => true,
        }
    }

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
#[derive(Clone)]
pub struct Matrix {
    /// Size defines the width and height of the matrix.
    pub size: usize,

    /// The modules.
    pub modules: Vec<Module>,
}

impl Matrix {
    /// Create a new matrix, modules initialized to false.
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

    /// Get mut module.
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
    pub fn set_fun(&mut self, x: usize, y: usize, v: bool) {
        let m = self.get_mut(x, y);
        assert!(!m.is_data());
        *m = Module::Function(v);
    }

    /// Assign a data module.
    pub fn set_data(&mut self, x: usize, y: usize, v: bool) {
        let m = self.get_mut(x, y);
        assert!(*m == Module::Unknown);
        *m = Module::Data(v);
    }

    /// Flip a module.
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

    ///// Returns true if the module at x,y is set.
    //pub fn is_set(&self, x: usize, y: usize) -> bool {
        //let i = self.index(x, y);
        //self.modules[i]
    //}

    ///// Set a module.
    //pub fn set(&mut self, x: usize, y: usize, v: bool) {
        //let i = self.index(x, y);
        //self.modules.set(i, v);
    //}

    ///// Flip module bit at x,y.
    //pub fn flip(&mut self, x: usize, y: usize) {
        //let i = self.index(x, y);
        //let v = !self.modules[i];
        //self.modules.set(i, v);
    //}

    ///// Set square outline.
    //pub fn set_square_outline(&mut self, x: usize, y: usize, w: usize, v: bool) {
        //// Above and below
        //for a in x..(x + w) {
            //self.set(a, y, v);
            //self.set(a, y + w - 1, v);
        //}
        //// Left and right
        //for b in (y + 1)..(y + w - 1) {
            //self.set(x, b, v);
            //self.set(x + w - 1, b, v);
        //}
    //}

    ///// Returns true if the module at x,y is a function.
    //pub fn is_fun(&self, x: usize, y: usize) -> bool {
        //let i = self.index(x, y);
        //self.functions[i]
    //}

    ///// Mark module as function.
    //pub fn mark_fun(&mut self, x: usize, y: usize) {
        //let i = self.index(x, y);
        //self.functions.set(i, true);
    //}

    ///// Mark modules in rect as functions.
    //pub fn mark_fun_square(&mut self, x: usize, y: usize, w: usize) {
        //self.mark_fun_rect(x, y, x + w - 1, y + w - 1);
    //}

    ///// Mark modules in rect as functions.
    //pub fn mark_fun_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        //for a in x0..(x1 + 1) {
            //for b in y0..(y1 + 1) {
                //self.mark_fun(a, b);
            //}
        //}
    //}

    ///// Return true if any module in a rect is marked as function
    //pub fn any_fun_in_square(&self, x: usize, y: usize, w: usize) -> bool {
        //for b in y..(y + w) {
            //for a in x..(x + w) {
                //if self.is_fun(a, b) {
                    //return true;
                //}
            //}
        //}
        //false
    //}
}

