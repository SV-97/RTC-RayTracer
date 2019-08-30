use crate::{
    primitives::{
        canvas::Canvas,
        pixel::Pixel,
        ray::Ray,
        rendering::Rendering,
        vector::{point, Transformation},
    },
    shading::{Color, Material, PointLight},
    shapes::{Shape, Sphere},
    utils::typelevel_nums::*,
};

use std::f64::consts;
use std::thread;

/// Let the canvas be at z=0
/// the sphere of radius 200 is in front of it at z = 250.
/// The sphere is in the center of the canvas at x = 450, y = 450.
/// The camera is another 400px further away at z = 650 and a bit
/// further up at y = 750.
/// The first lightsource is to the left of the camera.
pub fn sphere_rendering_parallel() -> std::io::Result<()> {
    let mut canvas = Canvas::<N900, N900>::new();
    let material = Material::new(Color::from((50, 70, 255)) * 0.8, 0.3, 0.7, 0.6, 1.);

    // Sphere
    let transforms = Transformation::new_scaling(200., 200., 200.).translated(450., 450., 250.);
    let sphere = Sphere::new(material, transforms);

    // Lights
    let l1 = PointLight::new(point(50., 650., 650.), Color::new_rgb(1.0, 1.0, 1.0));

    let camera = point(450., 450., 650.);
    let height = canvas.height();
    let xs = (0..canvas.width()).collect::<Vec<_>>();
    let n_threads = 4;

    let size = xs.len();
    let chunk_size = size / n_threads;
    let mut threads = Vec::with_capacity(n_threads);
    let chunks = xs.chunks(chunk_size);
    let results = {
        for (i, chunk) in chunks.enumerate() {
            let sphere = sphere.clone();
            let l1 = l1.clone();
            let camera = camera.clone();
            let chunk = chunk.iter().map(|x| *x).collect::<Vec<usize>>();
            threads.push(thread::spawn(move || {
                (i, {
                    let mut pixels = vec![];
                    for x in chunk.iter() {
                        let xf = *x as f64;
                        for y in 0..height {
                            let yf = ((height - 1) - y) as f64;
                            let r = Ray::new(
                                camera.clone(),
                                (point(xf, yf, 0.) - camera.clone()).unit(),
                            );
                            sphere.intersect(&r).map(|intersection| {
                                if let Some(hit) = intersection.hit() {
                                    let point = r.position(hit.t);
                                    let normal = sphere.normal_at(&point);
                                    let eye = -r.direction;
                                    let color =
                                        l1.lighting(&sphere.material, &point, &eye, &normal);
                                    pixels.push((*x, y, color));
                                }
                            });
                        }
                    }
                    pixels
                })
            }));
        }
        let mut out = Vec::with_capacity(n_threads);
        for thread in threads.into_iter() {
            let result = thread.join();
            out.push(result.unwrap());
        }
        out.sort_unstable_by(|(i1, _), (i2, _)| i1.partial_cmp(i2).unwrap());
        out.into_iter().map(|(_, e)| e).collect::<Vec<_>>()
    };

    for (x, y, pixel) in results.into_iter().flatten() {
        canvas.draw(x, y, pixel).expect("This should've happened.");
    }

    let r = Rendering::new("sphere_render", canvas);
    r.save_to_file()
}

/// Let the canvas be at z=0
/// the sphere of radius 200 is in front of it at z = 250.
/// The sphere is in the center of the canvas at x = 450, y = 450.
/// The camera is another 400px further away at z = 650 and a bit
/// further up at y = 750.
/// The first lightsource is to the left of the camera.
pub fn sphere_rendering() -> std::io::Result<()> {
    let mut canvas = Canvas::<N900, N900>::new();
    let material = Material::new(Color::from((50, 255, 60)) * 0.8, 0.3, 0.4, 0.6, 0.7);

    // Sphere
    let transforms = Transformation::new_scaling(200., 200., 200.).translated(450., 450., 250.);
    let sphere = Sphere::new(material, transforms);

    // Lights
    let l1 = PointLight::new(point(50., 650., 650.), Color::new_rgb(1.0, 1.0, 1.0));

    let camera = point(450., 450., 650.);
    for x in 0..canvas.width() {
        let xf = x as f64;
        for y in 0..canvas.height() {
            let yf = ((canvas.height() - 1) - y) as f64;
            let r = Ray::new(camera.clone(), (point(xf, yf, 0.) - camera.clone()).unit());
            sphere.intersect(&r).map(|intersection| {
                if let Some(hit) = intersection.hit() {
                    let point = r.position(hit.t);
                    let normal = sphere.normal_at(&point);
                    let eye = -r.direction;
                    let color = l1.lighting(&sphere.material, &point, &eye, &normal);
                    canvas.draw(x, y, color).expect("This should've happened.");
                }
            });
        }
    }
    let r = Rendering::new("sphere_render", canvas);
    r.save_to_file()
}
