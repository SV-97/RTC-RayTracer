use super::approx_eq::ApproxEq;
use super::pixels::Pixel;

use std::convert::TryInto;
use std::f32;
use std::ops::{Index, IndexMut};

pub struct Canvas {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    /// Convert a pair of coordinates on the canvas to an index into the pixel buffer
    pub fn as_one_dim(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    /// Draw a pixel to the canvas
    pub fn draw(&mut self, x: usize, y: usize, pixel: Pixel) -> Result<(), String> {
        if x < self.width {
            if y < self.height {
                let i = self.as_one_dim(x, y);
                self.pixels[i] = pixel;
                Ok(())
            } else {
                Err(format!(
                    "Tried accessing canvas out of bounds. Max y-index={}, actual index={}.",
                    self.height - 1,
                    y
                ))
            }
        } else {
            Err(format!(
                "Tried accessing canvas out of bounds. Max x-index={}, actual index={}.",
                self.width - 1,
                x
            ))
        }
    }

    /// Read a pixel at the given coordinate and check bounds
    pub fn get(&self, x: usize, y: usize) -> Result<Pixel, String> {
        if x < self.width {
            if y < self.height {
                let i = self.as_one_dim(x, y);
                Ok(self.pixels[i])
            } else {
                Err(format!(
                    "Tried accessing canvas out of bounds. Max y-index={}, actual index={}.",
                    self.height - 1,
                    y
                ))
            }
        } else {
            Err(format!(
                "Tried accessing canvas out of bounds. Max x-index={}, actual index={}.",
                self.width - 1,
                x
            ))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pixel> {
        self.pixels.iter()
    }

    /// Iterate over all the rows of the canvas
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Pixel>> {
        let mut v = vec![];
        for i in 0..self.height {
            v.push(self.iter().skip(i * self.width).take(self.width));
        }
        v.into_iter()
    }

    /// Iterate over all the columns of the canvas
    pub fn iter_columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Pixel>> {
        let mut v = vec![];
        for i in 0..self.width {
            v.push(self.iter().skip(i).step_by(self.width));
        }
        v.into_iter()
    }

    /// Return a PPM encoded version of the picture
    pub fn as_ppm(&self) -> String {
        let mut header = format!("P3\n{} {}\n255\n", self.width, self.height);

        let lines = self.iter_rows().fold(vec![], |mut buf, row| {
            let row_buf = row.fold(vec![], |mut row_buf, pixel| {
                row_buf.push(format!(
                    "{} {} {}",
                    clamp_and_normalize(pixel.r),
                    clamp_and_normalize(pixel.g),
                    clamp_and_normalize(pixel.b)
                ));
                row_buf
            });
            buf.push(row_buf.join(" "));
            buf
        });
        let length_verified_buf = lines
            .iter()
            .map(|s| split_long_lines(70, s))
            .map(|short_lines| short_lines.join("\n"))
            .collect::<Vec<String>>();
        let mut data = length_verified_buf.join("\n");
        format!("{}{}\n", header, data)
    }
}

/// Split lines that are over `max_length' long into multiple lines.
/// Breakes only at whitespace.
fn split_long_lines(max_length: usize, s: &String) -> Vec<String> {
    let mut length = 0;
    let mut line_buf = vec![];
    let mut buf = s.split_whitespace().fold(vec![], |mut buf, segment| {
        if length + segment.len() < max_length {
            length += segment.len() + 1;
            line_buf.push(segment);
        } else {
            buf.push(line_buf.join(" "));
            length = 0;
            line_buf.clear();
            length += segment.len();
            line_buf.push(segment);
        }
        buf
    });
    buf.push(line_buf.join(" "));
    buf
}

/// Clamp function, see https://github.com/rust-lang/rust/issues/44095
/// Unstable as of writing this (23.08.19)
pub fn clamp(num: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = num;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

/// Clamp the value to the range from 0 to 1 and then map that range onto 0 to 255
fn clamp_and_normalize(num: f32) -> u8 {
    ((clamp(num, 0.0, 1.0) * 255.0).round() as i64)
        .try_into()
        .unwrap()
}

impl Index<(usize, usize)> for Canvas {
    type Output = Pixel;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        let (x, y) = coords;
        &self.pixels[self.as_one_dim(x, y)]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut<'a>(&'a mut self, coords: (usize, usize)) -> &'a mut Self::Output {
        let (x, y) = coords;
        let i = self.as_one_dim(x, y);
        &mut self.pixels[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write() {
        let mut c = Canvas::new(2, 2);
        c[(0, 0)] = Pixel::white();
        c[(1, 0)] = Pixel::green();
        c[(1, 1)] = Pixel::red();
        assert!(c.pixels[0].approx_eq(Pixel::white()));
        assert!(c.pixels[1].approx_eq(Pixel::green()));
        assert!(c.pixels[2].approx_eq(Pixel::default()));
        assert!(c.pixels[3].approx_eq(Pixel::red()));
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(20, 10);
        assert_eq!(
            &c.as_ppm().lines().take(3).collect::<Vec<_>>().join("\n"),
            "P3\n20 10\n255"
        );
    }

    #[test]
    fn iter_rows() {
        let mut c = Canvas::new(2, 2);
        c[(0, 0)] = Pixel::white();
        c[(1, 0)] = Pixel::green();
        c[(1, 1)] = Pixel::red();
        let mut i = c.iter_rows();
        for p in c.iter_rows() {
            for p in p {
                dbg!(p);
            }
        }
        let mut row1 = i.next().unwrap();
        let mut row2 = i.next().unwrap();
        assert!(i.next().is_none());
        assert!(row1.next().unwrap().approx_eq(Pixel::white()));
        assert!(row1.next().unwrap().approx_eq(Pixel::green()));
        assert!(row2.next().unwrap().approx_eq(Pixel::default()));
        assert!(row2.next().unwrap().approx_eq(Pixel::red()));
    }
    #[test]
    fn ppm_data() {
        let mut c = Canvas::new(2, 2);
        c[(0, 0)] = Pixel::white() * 0.5;
        c[(1, 1)] = Pixel::red();
        c[(1, 0)] = Pixel::white() * 10.0;
        let s = c.as_ppm();
        assert_eq!(&s, "P3\n2 2\n255\n128 128 128 255 255 255\n0 0 0 255 0 0\n");
    }

    #[test]
    fn line_wrap() {
        let s = "123456789 123456789 123456789 123456789 123456789 123456789 123456789 abc defg 123456789 123456789 123456789 123456789 123456789 123456789 123456789 abc defg".to_string();
        assert_eq!(
            split_long_lines(70, &s),
            vec!(
                "123456789 123456789 123456789 123456789 123456789 123456789 123456789",
                "abc defg 123456789 123456789 123456789 123456789 123456789 123456789",
                "123456789 abc defg"
            )
        );
    }
}
