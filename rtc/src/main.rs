extern crate num_traits;

mod examples;
#[allow(dead_code)]
mod primitives;
mod utils;

fn main() {
    let _ = examples::projectile::simulate_trajectory().map_err(|e| println!("{}", e));
}
