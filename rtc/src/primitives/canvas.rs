use std::{
    convert::TryInto,
    f32,
    ops::{Index, IndexMut},
};

use crate::utils::{clamp, split_long_lines};

use super::pixel::Pixel;

pub struct Canvas {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![Pixel::default(); width * height];
        Canvas {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn to_row_major(&self, i: usize, j: usize) -> usize {
        self.width * i + j
    }

    pub fn draw_block(
        &mut self,
        x: usize,
        y: usize,
        half_width: usize,
        half_height: usize,
        pixel: Pixel,
    ) -> Vec<String> {
        let mut errs = vec![];
        for i in y - half_height..y + half_height {
            for j in x - half_width..x + half_width {
                if let Err(e) = self.draw(i, j, pixel) {
                    errs.push(e);
                }
            }
        }
        errs
    }

    /// Draw a pixel to the canvas
    pub fn draw(&mut self, x: usize, y: usize, pixel: Pixel) -> Result<(), String> {
        if x < self.width {
            if y < self.height {
                let i = self.to_row_major(y, x);
                self.data[i] = pixel;
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

    /// Return a PPM encoded version of the picture
    pub fn as_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);

        let lines = self.iter_rows().fold(vec![], |mut buf, row| {
            let row_buf = row.fold(vec![], |mut row_buf, pixel| {
                row_buf.push(format!(
                    "{} {} {}",
                    clamp_and_normalize(pixel.r, 255),
                    clamp_and_normalize(pixel.g, 255),
                    clamp_and_normalize(pixel.b, 255)
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
        let data = length_verified_buf.join("\n");
        format!("{}{}\n", header, data)
    }

    /// Iterate over all elements
    pub fn iter(&self) -> impl Iterator<Item = &Pixel> {
        self.data.iter()
    }

    /// Iterate over the ith row of the Canvas
    pub fn iter_row(&self, i: usize) -> impl Iterator<Item = &Pixel> {
        self.iter().skip(i * self.width).take(self.width)
    }

    /// Iterate over the jth coloumn of the Canvas
    pub fn iter_col(&self, j: usize) -> impl Iterator<Item = &Pixel> {
        self.iter().skip(j).step_by(self.width)
    }

    /// Iterate over all rows of the Canvas
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Pixel>> {
        (0..self.height).map(move |i| self.iter_row(i))
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Pixel;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        let (i, j) = coords;
        &self.data[self.to_row_major(i, j)]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        let (i, j) = coords;
        let idx = self.to_row_major(i, j);
        &mut self.data[idx]
    }
}

/// Clamp the value to the range from 0.0 to 1.0 and then map that range onto 0 to max
fn clamp_and_normalize(num: f32, max: usize) -> usize {
    ((clamp(num, 0.0, 1.0) * max as f32).round() as i64)
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::approx_eq::ApproxEq;

    #[test]
    fn read_and_write() {
        let mut c = Canvas::new(2, 2);
        c[(0, 0)] = Pixel::white();
        c[(0, 1)] = Pixel::green();
        c[(1, 1)] = Pixel::red();
        assert!(c.data[0].approx_eq(Pixel::white()));
        assert!(c.data[1].approx_eq(Pixel::green()));
        assert!(c.data[2].approx_eq(Pixel::default()));
        assert!(c.data[3].approx_eq(Pixel::red()));
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
        c[(0, 1)] = Pixel::green();
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
        c[(0, 1)] = Pixel::white() * 10.0;
        let s = c.as_ppm();
        assert_eq!(&s, "P3\n2 2\n255\n128 128 128 255 255 255\n0 0 0 255 0 0\n");
    }

    fn end_in_newline() {
        let c = Canvas::new(5, 3);
        assert_eq!(c.as_ppm().chars().last(), Some('\n'));
    }
}
