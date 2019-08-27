use std::convert::TryInto;
use std::f32;

use crate::utils::{clamp, split_long_lines, typelevel_nums::*};

use super::pixel::Pixel;
use super::tmatrix::Matrix;

pub type Canvas<WIDTH, HEIGHT> = Matrix<Pixel, HEIGHT, WIDTH>;

impl<WIDTH: Nat + Val, HEIGHT: Nat + Val> Canvas<WIDTH, HEIGHT> {
    /// Draw a pixel to the canvas
    pub fn draw(&mut self, x: usize, y: usize, pixel: Pixel) -> Result<(), String> {
        if x < WIDTH::val() {
            if y < HEIGHT::val() {
                let i = Self::to_row_major(y, x);
                self.data[i] = pixel;
                Ok(())
            } else {
                Err(format!(
                    "Tried accessing canvas out of bounds. Max y-index={}, actual index={}.",
                    HEIGHT::val() - 1,
                    y
                ))
            }
        } else {
            Err(format!(
                "Tried accessing canvas out of bounds. Max x-index={}, actual index={}.",
                WIDTH::val() - 1,
                x
            ))
        }
    }

    /// Return a PPM encoded version of the picture
    pub fn as_ppm(&self) -> String {
        let header = format!("P3\n{} {}\n255\n", WIDTH::val(), HEIGHT::val());

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
        let data = length_verified_buf.join("\n");
        format!("{}{}\n", header, data)
    }
}

/// Clamp the value to the range from 0 to 1 and then map that range onto 0 to 255
fn clamp_and_normalize(num: f32) -> u8 {
    ((clamp(num, 0.0, 1.0) * 255.0).round() as i64)
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::approx_eq::ApproxEq;

    #[test]
    fn read_and_write() {
        let mut c = Canvas::<N2, N2>::new();
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
        let c = Canvas::<N20, N10>::new();
        assert_eq!(
            &c.as_ppm().lines().take(3).collect::<Vec<_>>().join("\n"),
            "P3\n20 10\n255"
        );
    }

    #[test]
    fn iter_rows() {
        let mut c = Canvas::<N2, N2>::new();
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
        let mut c = Canvas::<N2, N2>::new();
        c[(0, 0)] = Pixel::white() * 0.5;
        c[(1, 1)] = Pixel::red();
        c[(0, 1)] = Pixel::white() * 10.0;
        let s = c.as_ppm();
        assert_eq!(&s, "P3\n2 2\n255\n128 128 128 255 255 255\n0 0 0 255 0 0\n");
    }

    fn end_in_newline() {
        let c = Canvas::<N5, N3>::new();
        assert_eq!(c.as_ppm().chars().last(), Some('\n'));
    }
}
