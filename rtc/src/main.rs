#![recursion_limit = "4096"]
#![allow(clippy::cast_lossless)]
extern crate num_traits;

mod examples;
#[allow(dead_code)]
mod primitives;
mod profile;
mod scenes;
mod shading;
mod shapes;
mod utils;

fn main() {
    let _ = examples::world_rendering::world_rendering_3().map_err(|e| println!("{}", e));
}
