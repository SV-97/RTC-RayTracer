use std::fs::File;
use std::io::prelude::*;

use crate::utils::typelevel_nums::*;
use super::canvas::Canvas;

pub struct Rendering<HEIGHT: Nat, WIDTH: Nat> {
    name: String,
    canvas: Canvas<HEIGHT, WIDTH>,
}

impl<HEIGHT: Nat + Val, WIDTH: Nat + Val> Rendering<HEIGHT, WIDTH> {
    pub fn new(name: impl Into<String>, canvas: Canvas<HEIGHT, WIDTH>) -> Self {
        Rendering {
            name: name.into(),
            canvas,
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create(&format!("renders/{}.ppm", self.name))?;
        file.write_all(self.canvas.as_ppm().as_bytes())?;
        Ok(())
    }
}
