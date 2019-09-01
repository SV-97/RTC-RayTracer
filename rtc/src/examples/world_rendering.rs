use crate::{
    primitives::{
        canvas::Canvas,
        pixel::Pixel,
        ray::Ray,
        rendering::Rendering,
        transformation_matrices,
        vector::{point, vector, Point, Transformation},
    },
    scenes::{Camera, World},
    shading::{Color, Material, PointLight},
    shapes::{Shape, Sphere},
    utils::typelevel_nums::*,
};

use std::f64::consts;

pub fn world_rendering_1() -> std::io::Result<()> {
    let green = Material::new(Color::from((50, 255, 60)) * 0.8, 0.3, 0.4, 0.6, 200.);
    let grey = Material::new(Color::from((50, 50, 50)), 0.3, 0.6, 0.4, 0.7);
    let purpleish = Material::new(Color::from((220, 20, 220)) * 0.5, 0.3, 0.5, 0.4, 1000.);
    let base_mat = Material::new(Color::from((30, 30, 30)), 0.3, 0.5, 0., 50.);

    let arrow_red = Material::new(Color::red(), 0.3, 0.5, 0., 500.);
    let arrow_green = Material::new(Color::green(), 0.3, 0.5, 0., 500.);
    let arrow_blue = Material::new(Color::blue(), 0.3, 0.5, 0., 500.);

    let world = World::new(
        vec![
            Sphere::new(green, Transformation::new_translation(6., 5., 1.)),
            Sphere::new(
                grey,
                Transformation::new_scaling(3., 3., 3.).translated(3., 3., 3.),
            ),
            Sphere::new(
                purpleish,
                Transformation::new_scaling(4., 1., 1.)
                    .rotated_z(consts::FRAC_PI_2)
                    .translated(5., 5., 7.),
            ),
            // floors and walls
            Sphere::new(base_mat, Transformation::new_scaling(1000., 1000., 0.01)),
            Sphere::new(base_mat, Transformation::new_scaling(0.01, 1000., 1000.)),
            Sphere::new(base_mat, Transformation::new_scaling(1000., 0.01, 1000.)),
            // arrow thingies
            Sphere::new(arrow_blue, Transformation::new_scaling(1000., 0.1, 0.1)),
            Sphere::new(arrow_green, Transformation::new_scaling(0.1, 1000., 0.1)),
            Sphere::new(arrow_red, Transformation::new_scaling(0.1, 0.1, 1000.)),
        ],
        vec![
            PointLight::new(point(10., 10., 1.), Color::white() * 2.),
            // PointLight::new(point(-10., 0., 10.), Color::from((10, 1, 1)) * 5.),
        ],
    );

    let from = point(12., 5., 15.);
    let to = point(3., 4., 0.);
    let up = vector(0., 1., 0.);

    let camera =
        //Camera::<N1024, N900>::new(consts::FRAC_PI_2, Transformation::new_view(&from, &to, &up));
        Camera::new(400, 400, consts::FRAC_PI_2, Transformation::new_view(&from, &to, &up));

    let canvas = camera.render(world);

    let r = Rendering::new("world_render_1", canvas);
    r.save_to_file()
}

pub fn world_rendering_2() -> std::io::Result<()> {
    let grey = Material::new(Color::from((50, 50, 50)), 0.3, 0.6, 0.4, 1000.);
    let base_mat = Material::new(Color::from((70, 70, 70)), 0.3, 0.5, 0., 50.);

    let arrow_red = Material::new(Color::red(), 0.3, 0.5, 0., 500.);
    let arrow_green = Material::new(Color::green(), 0.3, 0.5, 0., 500.);
    let arrow_blue = Material::new(Color::blue(), 0.3, 0.5, 0., 500.);

    let light_setup = Transformation::new_translation(3., 12., 0.).rotated_y(0.785398);
    let light_move = Transformation::new_translation(6., 0., 6.);
    let l1 = &light_move * &light_setup * Point::origin();
    let l2 = &light_move * (&light_setup * Point::origin()).rotated_y(consts::FRAC_PI_3 * 2.0);
    let l3 = &light_move * (&light_setup * Point::origin()).rotated_y(consts::FRAC_PI_3 * 4.0);

    let world = World::new(
        vec![
            Sphere::new(grey, Transformation::new_translation(6., 3., 6.)),
            Sphere::new(
                grey,
                Transformation::new_scaling(1., 3., 1.).translated(6., 8., 6.),
            ),
            // floors and walls
            Sphere::new(base_mat, Transformation::new_scaling(1000., 1000., 0.01)),
            Sphere::new(base_mat, Transformation::new_scaling(0.01, 1000., 1000.)),
            Sphere::new(base_mat, Transformation::new_scaling(1000., 0.01, 1000.)),
            // arrow thingies
            Sphere::new(arrow_blue, Transformation::new_scaling(1000., 0.1, 0.1)),
            Sphere::new(arrow_green, Transformation::new_scaling(0.1, 1000., 0.1)),
            Sphere::new(arrow_red, Transformation::new_scaling(0.1, 0.1, 1000.)),
        ],
        vec![
            PointLight::new(point(6., 13., 6.), Color::white() * 0.8),
            PointLight::new(l1, Color::from((100, 20, 20)) * 3.),
            PointLight::new(l2, Color::from((20, 20, 100)) * 3.),
            PointLight::new(l3, Color::from((20, 100, 20)) * 3.),
        ],
    );

    let from = point(12., 6., 12.);
    let to = point(0., 4., 0.);
    let up = vector(0., 1., 0.);

    let camera = Camera::new(
        4096,
        2160,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );

    let canvas = camera.render(world);

    let r = Rendering::new("world_render_2", canvas);
    r.save_to_file()
}
