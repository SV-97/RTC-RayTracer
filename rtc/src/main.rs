#![recursion_limit = "4096"]
#![allow(clippy::cast_lossless)]
#![allow(dead_code)]
extern crate num_traits;

mod examples;
mod primitives;
mod profile;
mod scenes;
mod shading;
mod shapes;
mod utils;

fn main() {
    let _ = examples::world_rendering::world_rendering_8().map_err(|e| println!("{}", e));
}
