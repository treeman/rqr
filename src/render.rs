//! Renders the QR code to different outputs.
//!
//! Outputs to a string representation and svg are supported.
use crate::matrix::{Matrix, Module};
use crate::qr::Qr;

use std::str::FromStr;
use std::num::ParseIntError;
use std::u8;

/// A string renderer for converting a QR code into a representation
/// suitable for text output.
pub struct StringRenderer {
    light: char,
    dark: char,
    module_w: usize,
    module_h: usize,
    qz: bool,
}

impl StringRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            light: '.',
            dark: '#',
            module_w: 1,
            module_h: 1,
            qz: false,
        }
    }

    /// Set the light module character.
    pub fn light_module(mut self, v: char) -> Self {
        self.light = v;
        self
    }

    /// Set the dark module character.
    pub fn dark_module(mut self, v: char) -> Self {
        self.dark = v;
        self
    }

    /// Set if quiet zone should be produced.
    pub fn quiet_zone(mut self, v: bool) -> Self {
        self.qz = v;
        self
    }

    /// Set the module dimensions, in character count per module.
    pub fn module_dimensions(mut self, w: usize, h: usize) -> Self {
        assert!(w > 0 && h > 0);
        self.module_w = w;
        self.module_h = h;
        self
    }

    /// Render QR to string.
    pub fn render(&self, qr: &Qr) -> String {
        self.render_matrix(&qr.matrix)
    }

    /// Render matrix to string.
    pub fn render_matrix(&self, matrix: &Matrix) -> String {
        let mut res = String::with_capacity(matrix.size * matrix.size);
        self.qz_lines(&mut res);
        for y in 0..matrix.size {
            // Duplicate rows for larger module dimensions.
            for _ in 0..self.module_h {
                let mut s = String::with_capacity(matrix.size + 1);
                self.qz_chars(&mut s);
                for x in 0..matrix.size {
                    let c = if matrix.is_dark(x, y) {
                        self.dark
                    } else {
                        self.light
                    };
                    // Duplicate chars for larger module dimensions.
                    for _ in 0..self.module_w {
                        s.push(c);
                    }
                }
                self.qz_chars(&mut s);
                s.push('\n');
                res.push_str(&s);
            }
        }
        self.qz_lines(&mut res);
        res
    }

    // Append empty lines for quiet zone padding.
    fn qz_lines(&self, s: &mut String) {
        if self.qz {
            for _ in 0..(4*self.module_h) {
                s.push_str("\n");
            }
        }
    }

    // Append whitespace chars for quiet zone padding.
    fn qz_chars(&self, s: &mut String) {
        if self.qz {
            for _ in 0..(4*self.module_w) {
                s.push(' ');
            }
        }
    }
}

/// Convert to string, with chars for the different underlying representations.
pub fn to_dbg_string(matrix: &Matrix) -> String {
    let mut res = String::with_capacity(matrix.size * matrix.size);
    res.push('\n');
    for y in 0..matrix.size {
        let mut s = String::with_capacity(matrix.size + 1);
        for x in 0..matrix.size {
            let c = match matrix.get(x, y) {
                Module::Unknown => '?',
                Module::Reserved => '*',
                Module::Function(true) => '#',
                Module::Function(false) => '.',
                Module::Data(true) => 'X',
                Module::Data(false) => '-',
            };
            s.push(c);
        }
        s.push('\n');
        res.push_str(&s);
    }
    res
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// An RGB color implementation.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new from rgb parts.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new color from a hex input.
    /// ```
    /// # use rqr::*;
    /// let c = Color::hex(0xff3214);
    /// ```
    pub fn hex(v: u32) -> Self {
        Self {
            r: (v >> 16) as u8,
            g: (v >> 8) as u8,
            b: v as u8,
        }
    }

    /// Create a new color from a length 4 input hex.
    /// ```
    /// # use rqr::*;
    /// // Short for "#770000"
    /// let c = Color::from_4_hex("#700");
    /// ```
    pub fn from_4_hex(s: &str) -> Result<Self, ParseColorError> {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] != '#' {
            return Err(ParseColorError);
        }
        let r = u8::from_str_radix(&chars[1].to_string(), 16)?;
        let g = u8::from_str_radix(&chars[2].to_string(), 16)?;
        let b = u8::from_str_radix(&chars[3].to_string(), 16)?;
        Ok(Color {
            r: (r << 4) | r,
            g: (g << 4) | g,
            b: (b << 4) | b
        })
    }

    /// Create a new color from a length 7 input hex.
    /// ```
    /// # use rqr::*;
    /// let c = Color::from_7_hex("#3477ff");
    /// ```
    pub fn from_7_hex(s: &str) -> Result<Self, ParseColorError> {
        if s[0..1] != *"#" {
            return Err(ParseColorError);
        }
        let r = u8::from_str_radix(&s[1..3], 16)?;
        let g = u8::from_str_radix(&s[3..5], 16)?;
        let b = u8::from_str_radix(&s[5..7], 16)?;
        Ok(Color {
            r,
            g,
            b
        })
    }

    /// Convert to a hex string.
    /// ```
    /// # use rqr::*;
    /// assert_eq!(Color::hex(0xff7312).to_hex_str(), "#ff7312");
    /// ```
    pub fn to_hex_str(&self) -> String {
        String::from(format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b))
    }
}

#[derive(Debug, Copy, Clone)]
/// An error from trying to parse a Color instance from string.
pub struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            4 => Color::from_4_hex(s),
            7 => Color::from_7_hex(s),
            _ => Err(ParseColorError),
        }
    }

}

impl From <ParseIntError> for ParseColorError {
    fn from(_: ParseIntError) -> Self {
        ParseColorError
    }
}

/// A string renderer for converting a QR code into svg.
pub struct SvgRenderer {
    light: Color,
    dark: Color,
    w: usize,
    h: usize,
    qz: bool,
}

impl SvgRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            light: Color::new(255, 255, 255),
            dark: Color::new(0, 0, 0),
            w: 200,
            h: 200,
            qz: true,
        }
    }

    /// Set the light module color.
    /// Will also be the color of the quiet zone, if relevant.
    pub fn light_module(mut self, v: Color) -> Self {
        self.light = v;
        self
    }

    /// Set the dark module color.
    pub fn dark_module(mut self, v: Color) -> Self {
        self.dark = v;
        self
    }

    /// Set if quiet zone should be produced.
    pub fn quiet_zone(mut self, v: bool) -> Self {
        self.qz = v;
        self
    }

    /// Set the dimensions of the output, in pixels.
    /// Includes the quiet zone, if relevant.
    pub fn dimensions(mut self, w: usize, h: usize) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    /// Render QR.
    pub fn render(&self, qr: &Qr) -> String {
        self.render_matrix(&qr.matrix)
    }

    /// Render matrix.
    pub fn render_matrix(&self, matrix: &Matrix) -> String {
        let cell_count = if self.qz { matrix.size + 8 } else { matrix.size };
        // If not divided evenly adjust upwards and treat specified
        // width and height as minimums.
        let cell_w = ((self.w as f64) / (cell_count as f64)).ceil() as usize;
        let cell_h = ((self.h as f64) / (cell_count as f64)).ceil() as usize;
        // We might grow larger so readjust dimensions.
        let w = cell_w * cell_count;
        let h = cell_h * cell_count;

        let mut res = String::from(format!(
"<?xml version=\"1.0\" standalone=\"yes\"?>
<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\"
    viewBox=\"0 0 {w} {h}\" shape-rendering=\"crispEdges\">
<rect x=\"0\" y=\"0\" width=\"{w}\" height=\"{h}\" fill=\"{light}\"/>
<path fill=\"{dark}\" d=\"",
        w = w,
        h = h,
        light = self.light.to_hex_str(),
        dark = self.dark.to_hex_str()));

        for y in 0..matrix.size {
            let yp = if self.qz { (y + 4) * cell_h } else { y * cell_h };

            for x in 0..matrix.size {
                let xp = if self.qz { (x + 4) * cell_w } else { x * cell_w };

                if matrix.is_dark(x, y) {
                    res.push_str(format!("M{x} {y}h{w}v{h}H{x}V{y}",
                                        x = xp,
                                        y = yp,
                                        w = cell_w,
                                        h = cell_h).as_str());
                }
            }
        }
        res.push_str("\"/></svg>\n");
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::QrBuilder;
    use crate::version::Version;
    use crate::ec::ECLevel;

    #[test]
    fn string_renderer() {
        let mut builder = QrBuilder::new()
            .version(Version::new(1))
            .ecl(ECLevel::Q);
        builder.add_all("HELLO WORLD").unwrap();
        let s = StringRenderer::new()
            .light_module('~')
            .dark_module('X')
            .module_dimensions(2, 1)
            .render_matrix(&builder.matrix);
        //println!("{}", s);
        let expected =
"XXXXXXXXXXXXXX~~~~~~~~XX~~~~XXXXXXXXXXXXXX
XX~~~~~~~~~~XX~~XXXX~~~~XX~~XX~~~~~~~~~~XX
XX~~XXXXXX~~XX~~~~XX~~XXXX~~XX~~XXXXXX~~XX
XX~~XXXXXX~~XX~~XXXXXXXXXX~~XX~~XXXXXX~~XX
XX~~XXXXXX~~XX~~XXXX~~XX~~~~XX~~XXXXXX~~XX
XX~~~~~~~~~~XX~~~~XX~~~~XX~~XX~~~~~~~~~~XX
XXXXXXXXXXXXXX~~XX~~XX~~XX~~XXXXXXXXXXXXXX
~~~~~~~~~~~~~~~~XXXX~~XXXX~~~~~~~~~~~~~~~~
~~XX~~XXXXXXXX~~XXXX~~~~XXXXXX~~XXXX~~XX~~
XX~~XXXXXXXX~~XX~~~~~~~~XXXXXXXX~~XXXXXX~~
~~~~XX~~XX~~XXXX~~~~~~XX~~~~XXXX~~~~~~~~~~
XX~~XXXX~~XX~~~~~~XX~~XXXX~~~~~~XXXX~~~~~~
XXXX~~XXXXXXXXXXXXXXXX~~XXXXXX~~XXXXXXXXXX
~~~~~~~~~~~~~~~~XX~~~~~~XX~~~~XX~~XX~~~~~~
XXXXXXXXXXXXXX~~~~XXXX~~~~XXXX~~~~XXXXXXXX
XX~~~~~~~~~~XX~~XX~~XX~~~~XX~~~~XX~~XXXXXX
XX~~XXXXXX~~XX~~XXXX~~XX~~~~XX~~~~~~XXXXXX
XX~~XXXXXX~~XX~~XX~~XXXXXX~~~~~~XX~~XX~~~~
XX~~XXXXXX~~XX~~~~XX~~~~~~~~XX~~~~~~~~XXXX
XX~~~~~~~~~~XX~~XXXXXX~~~~XXXXXX~~~~XXXX~~
XXXXXXXXXXXXXX~~~~XX~~XX~~~~~~~~~~~~~~XX~~
";
        assert_eq!(s, expected);
    }

    #[test]
    fn svg_renderer() {
        let mut builder = QrBuilder::new()
            .version(Version::new(1))
            .ecl(ECLevel::Q);
        builder.add_all("HELLO WORLD").unwrap();
        let s = SvgRenderer::new()
            .light_module(Color::new(229, 189, 227))
            .dark_module(Color::new(119, 0, 0))
            .dimensions(200, 200)
            .render_matrix(&builder.matrix);
        //println!("{}", s);
        let expected = include_str!("test/hello_world.svg");
        assert_eq!(s, expected);
    }

    #[test]
    fn color() {
        assert_eq!(Color::new(255, 100, 32), Color::hex(0xff6420));
        assert_eq!(Color::hex(0xff6420).to_hex_str(), "#ff6420");
        assert_eq!(Color::new(255, 0, 0).to_hex_str(), "#ff0000");
        assert_eq!("#f92".parse::<Color>().unwrap(),
                   Color::hex(0xff9922));
        assert_eq!("#f1a293".parse::<Color>().unwrap(),
                   Color::hex(0xf1a293));
        assert!("#ooo".parse::<Color>().is_err())
    }
}

