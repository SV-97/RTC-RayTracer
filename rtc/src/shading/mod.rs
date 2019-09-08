use crate::primitives::pixel::Pixel;

pub type Color = Pixel;

pub use builtin_materials::*;
pub use builtin_patterns::*;
pub use lights::*;
pub use material::*;
pub use pattern::*;

mod builtin_materials;
mod builtin_patterns;
mod lights;
mod material;
mod pattern;

#[cfg(test)]
mod tests;
