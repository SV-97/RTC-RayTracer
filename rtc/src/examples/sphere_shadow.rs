use crate::{
    primitives::{
        canvas::Canvas,
        pixel::Pixel,
        ray::Ray,
        rendering::Rendering,
        vector::{point, Transformation},
    },
    shapes::{Shape, Sphere},
    utils::typelevel_nums::*,
};
use std::f64::consts;

/// Let the canvas be at z=0
/// the sphere of radius 200 is in front of it at z = 250
/// the sphere is in the center of the canvas at x = 450, y = 450
/// the camera or lightsource is another 400px further away at z = 650
pub fn sphere_shadow() -> std::io::Result<()> {
    let mut canvas = Canvas::<N900, N900>::new();
    let mut sphere = Sphere::default();
    let transforms = Transformation::new_scaling(200., 200., 200.).and_translate(450., 450., 250.);

    sphere.set_transform(transforms);
    let pen = Pixel::from((40, 200, 220));
    let background = Pixel::from((10, 10, 10));
    let light_source = point(450., 450., 650.);
    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            let xf = x as f64;
            let yf = y as f64;
            let r = Ray::new(
                light_source.clone(),
                point(xf, yf, 0.) - light_source.clone(),
            );
            let is = sphere.intersect(&r);
            match is {
                Some(is) => {
                    if is.hit().is_some() {
                        canvas.draw(x, y, pen).expect("This should've happened.");
                    } else {
                        canvas
                            .draw(x, y, background)
                            .expect("This should've happened.");
                    }
                }
                _ => canvas
                    .draw(x, y, background)
                    .expect("This should've happened."),
            }
        }
    }
    let r = Rendering::new("sphere_shadow", canvas);
    r.save_to_file()
}
