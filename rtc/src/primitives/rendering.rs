use std::fs::File;
use std::io::prelude::*;

use super::canvas::Canvas;

pub struct Rendering {
    name: String,
    canvas: Canvas,
}

impl Rendering {
    pub fn new(name: impl Into<String>, canvas: Canvas) -> Self {
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
