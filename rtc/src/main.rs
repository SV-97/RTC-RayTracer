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
    // let _ = examples::projectile::simulate_trajectory().map_err(|e| println!("{}", e));
    // let _ = examples::clock::draw_clockface().map_err(|e| println!("{}", e));
    // let _ = examples::sphere_shadow::sphere_shadow().map_err(|e| println!("{}", e));
    // let _ = examples::sphere_rendering::sphere_rendering().map_err(|e| println!("{}", e));
    let _ = examples::world_rendering::world_rendering_2().map_err(|e| println!("{}", e));
}
