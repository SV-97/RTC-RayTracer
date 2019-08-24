use crate::primitives::{
    point::Point,
    vector::Vec3D,
    canvas::Canvas,
    pixel::Pixel,
    rendering::Rendering,
};

/// Creates and image of a projectile simulation
pub fn simulate_trajectory() -> std::io::Result<()> {
    let sim = Simulation::new(
        Environment::new(
            Vec3D::new(0.0, -0.1, 0.0),
            Vec3D::new(-0.01, 0.0, 0.0)),
        Projectile::new(
            Point::new(0.0, 1.0, 0.0),
            Vec3D::new(1.0, 1.8, 0.0).unit() * 11.25),
            );
    let mut canvas = Canvas::new(900, 550);
    let pen = Pixel::from((253, 150, 20));
    for point in sim.take_while(|p| p.position.y > 0.0) {
        let x = point.position.x.round() as usize;
        let y = canvas.height - point.position.y.round() as usize;
        for i in x-2..x+2 {
            for j in y-2..y+2 {
                let _ = canvas.draw(i, j, pen).map_err(|e| println!("{}", e));
            }
        }
    }
    let r = Rendering::new("trajectory", canvas);
    r.save_to_file()
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Projectile {
    position: Point,
    velocity: Vec3D,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vec3D) -> Self {
        Projectile { position, velocity, }
    }
}

struct Environment {
    gravity: Vec3D,
    wind: Vec3D,
}

impl Environment {
    pub fn new(gravity: Vec3D, wind: Vec3D) -> Self {
        Environment { gravity, wind }
    }

    pub fn tick(&self, p: &mut Projectile) {
        p.position += p.velocity;
        p.velocity += self.gravity + self.wind;
    }
}

struct Simulation {
    env: Environment,
    projectile: Projectile,
}

impl Simulation {
    pub fn new( env: Environment, projectile: Projectile ) -> Self {
        Simulation { env, projectile }
    }
}

impl Iterator for Simulation {
    type Item = Projectile;
    fn next(&mut self) -> Option<Self::Item> {
        self.env.tick(&mut self.projectile);
        Some(self.projectile)
    }
}
