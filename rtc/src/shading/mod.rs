use crate::primitives::pixel::Pixel;

pub type Color = Pixel;

pub use lights::*;
pub use material::*;

mod lights;
mod material;

// #[cfg(test)]
// mod tests;
