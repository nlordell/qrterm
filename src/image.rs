//! Image implementation for rendring QR codes to terminals. Note that we create
//! a "point" abstraction for representing two QR dots. This is done because
//! terminal characters are vertical and can display two QR dots. This means
//! that when rendering, we print out two dots at a time per character.

use qrcode::render::{Canvas, Pixel};
use qrcode::types::Color;

/// A QR dot that can either be white or black.
#[derive(Clone, Copy)]
pub enum Dot {
    /// A black dot.
    Black,
    /// A white dot.
    White,
}

impl Pixel for Dot {
    type Canvas = Grid;
    type Image = Image;

    fn default_color(color: Color) -> Self {
        match color {
            Color::Light => Dot::White,
            Color::Dark => Dot::Black,
        }
    }

    fn default_unit_size() -> (u32, u32) {
        (1, 1)
    }
}

/// A rendering character. This is slightly different than a dot as terminal
/// characters have a height of two dots.
pub struct Point {
    pub top: Dot,
    pub bot: Dot,
}

impl Point {
    /// Converts a point to a unicode block character.
    ///
    /// Note this method assume `Black` to be filled in, meaning it will look
    /// "correct" when using a white background and black font colour.
    pub fn to_char(&self) -> char {
        match (self.top, self.bot) {
            (Dot::Black, Dot::Black) => '█',
            (Dot::Black, Dot::White) => '▀',
            (Dot::White, Dot::Black) => '▄',
            (Dot::White, Dot::White) => ' ',
        }
    }
}

/// A half point, when there is an uneven number of rows. The distiction is
/// important when using 256 colors where the true black is different than
/// terminal background off-black.
pub struct HalfPoint(pub Dot);

impl HalfPoint {
    /// Converts a half point to a unicode block character.
    ///
    /// See [`Point::to_char`] for more details.
    pub fn to_char(&self) -> char {
        match self.0 {
            Dot::Black => '▀',
            Dot::White => ' ',
        }
    }
}

/// A image grid used for rendering.
pub struct Grid {
    dots: Vec<Dot>,
    width: usize,
    dark: Dot,
}

impl Canvas for Grid {
    type Pixel = Dot;
    type Image = Image;

    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        let (w, h) = (width as usize, height as usize);
        Grid {
            dots: vec![light_pixel; w * h],
            width: w,
            dark: dark_pixel,
        }
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        let (x, y) = (x as usize, y as usize);

        let i = x + y * self.width;
        if x >= self.width || i >= self.dots.len() {
            panic!("pixel out of bounds!")
        }

        self.dots[i] = self.dark;
    }

    fn into_image(self) -> Self::Image {
        let w = self.width;
        let h = self.dots.len() / w;
        let mut lines = Vec::with_capacity(h / 2);
        let mut last_line = None;

        for line in self.dots.chunks(w * 2) {
            if line.len() == w * 2 {
                lines.push(
                    line[..w]
                        .iter()
                        .zip(&line[w..])
                        .map(|(&top, &bot)| Point { top, bot })
                        .collect(),
                );
            } else {
                last_line = Some(line.iter().copied().map(HalfPoint).collect())
            }
        }

        Image { lines, last_line }
    }
}

/// A QR image for rendering to the terminal.
pub struct Image {
    pub lines: Vec<Vec<Point>>,
    pub last_line: Option<Vec<HalfPoint>>,
}
