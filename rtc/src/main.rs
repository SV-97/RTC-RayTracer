#[allow(dead_code)]
mod primitives;
mod examples;

fn main() {
    let _ = examples::projectile::simulate_trajectory().map_err(|e| println!("{}", e));
}
