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
    shading::*,
    shapes::{Shape, SPHERE},
    utils::typelevel_nums::*,
};

use std::f64::consts;

pub fn world_rendering_1() -> std::io::Result<()> {
    let green = Material::new(
        Color::from((50, 255, 60)) * 0.8,
        0.3,
        0.4,
        0.6,
        200.,
        0.,
        0.,
        1.,
    );
    let grey = Material::new(Color::from((50, 50, 50)), 0.3, 0.6, 0.4, 0.7, 0., 0., 1.);
    let purpleish = Material::new(
        Color::from((220, 20, 220)) * 0.5,
        0.3,
        0.5,
        0.4,
        1000.,
        0.,
        0.,
        1.,
    );
    let base_mat = Material::new(Color::from((30, 30, 30)), 0.3, 0.5, 0., 50., 0., 0., 1.);

    let arrow_red = Material::new(Color::red(), 0.3, 0.5, 0., 500., 0., 0., 1.);
    let arrow_green = Material::new(Color::green(), 0.3, 0.5, 0., 500., 0., 0., 1.);
    let arrow_blue = Material::new(Color::blue(), 0.3, 0.5, 0., 500., 0., 0., 1.);

    let world = World::new(
        vec![
            Shape::new_sphere(green, Transformation::new_translation(6., 5., 1.)),
            Shape::new(
                SPHERE,
                grey,
                Transformation::new_scaling(3., 3., 3.).translated(3., 3., 3.),
            ),
            Shape::new(
                SPHERE,
                purpleish,
                Transformation::new_scaling(4., 1., 1.)
                    .rotated_z(consts::FRAC_PI_2)
                    .translated(5., 5., 7.),
            ),
            // floors and walls
            Shape::new(
                SPHERE,
                base_mat.clone(),
                Transformation::new_scaling(1000., 1000., 0.01),
            ),
            Shape::new(
                SPHERE,
                base_mat.clone(),
                Transformation::new_scaling(0.01, 1000., 1000.),
            ),
            Shape::new(
                SPHERE,
                base_mat.clone(),
                Transformation::new_scaling(1000., 0.01, 1000.),
            ),
            // arrow thingies
            Shape::new(
                SPHERE,
                arrow_blue,
                Transformation::new_scaling(1000., 0.1, 0.1),
            ),
            Shape::new(
                SPHERE,
                arrow_green,
                Transformation::new_scaling(0.1, 1000., 0.1),
            ),
            Shape::new(
                SPHERE,
                arrow_red,
                Transformation::new_scaling(0.1, 0.1, 1000.),
            ),
        ],
        vec![PointLight::new(point(10., 10., 1.), Color::white() * 2.)],
    );

    let from = point(12., 5., 15.);
    let to = point(3., 4., 0.);
    let up = vector(0., 1., 0.);

    let camera = Camera::new(
        400,
        400,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );

    let canvas = camera.render(world);

    let r = Rendering::new("world_render_1", canvas);
    r.save_to_file()
}

pub fn world_rendering_2() -> std::io::Result<()> {
    let grey = Material::new(Color::from((50, 50, 50)), 0.3, 0.6, 0.4, 1000., 0., 0., 1.);
    let base_mat = Material::new(Color::from((70, 70, 70)), 0.3, 0.5, 0., 50., 0.5, 0., 1.);

    let arrow_red = Material::new(Color::red(), 0.3, 0.5, 0., 500., 0., 0., 1.);
    let arrow_green = Material::new(Color::green(), 0.3, 0.5, 0., 500., 0., 0., 1.);
    let arrow_blue = Material::new(Color::blue(), 0.3, 0.5, 0., 500., 0., 0., 1.);

    let light_setup = Transformation::new_translation(3., 12., 0.).rotated_y(consts::FRAC_PI_4);
    let light_move = Transformation::new_translation(6., 0., 6.);
    let l1 = &light_move * &light_setup * Point::origin();
    let l2 = &light_move * (&light_setup * Point::origin()).rotated_y(consts::FRAC_PI_3 * 2.0);
    let l3 = &light_move * (&light_setup * Point::origin()).rotated_y(consts::FRAC_PI_3 * 4.0);

    let world = World::new(
        vec![
            Shape::new_sphere(grey.clone(), Transformation::new_translation(6., 3., 6.)),
            Shape::new(
                SPHERE,
                grey.clone(),
                Transformation::new_scaling(1., 3., 1.).translated(6., 8., 6.),
            ),
            // floors and walls
            Shape::new(
                SPHERE,
                base_mat.clone(),
                Transformation::new_scaling(1000., 1000., 0.01),
            ),
            Shape::new(
                SPHERE,
                base_mat.clone(),
                Transformation::new_scaling(0.01, 1000., 1000.),
            ),
            Shape::new(
                SPHERE,
                base_mat.clone(),
                Transformation::new_scaling(1000., 0.01, 1000.),
            ),
            // arrow thingies
            Shape::new(
                SPHERE,
                arrow_blue,
                Transformation::new_scaling(1000., 0.1, 0.1),
            ),
            Shape::new(
                SPHERE,
                arrow_green,
                Transformation::new_scaling(0.1, 1000., 0.1),
            ),
            Shape::new(
                SPHERE,
                arrow_red,
                Transformation::new_scaling(0.1, 0.1, 1000.),
            ),
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
        2048,
        1080,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );

    let canvas = camera.render(world);

    let r = Rendering::new("world_render_2", canvas);
    r.save_to_file()
}

pub fn world_rendering_3() -> std::io::Result<()> {
    let blue = Material::new(Color::from((30, 0, 200)), 0.3, 0.2, 0.1, 1., 0., 0., 1.);
    let whiteish = Material::new(Color::from((50, 50, 70)), 0.1, 0.3, 0.3, 1., 0., 0., 1.);
    let space_blue = Material::new(Color::from((0, 0, 20)), 0.8, 0.5, 0., 50., 0., 0., 1.);

    let frames = 36;
    for i in 0..frames {
        let angle = consts::PI * 2. * i as f64 / frames as f64;
        let world = World::new(
            vec![
                Shape::new_sphere(blue.clone(), Transformation::new_scaling(5., 5., 5.)),
                Shape::new_sphere(
                    whiteish.clone(),
                    Transformation::new_translation(-5., 3., 2.).rotated_y(angle),
                ),
                Shape::new_plane(space_blue.clone(), Transformation::identity()),
            ],
            vec![PointLight::new(point(-100., 10., 0.), Color::white() * 3.8)],
        );

        let from = point(12., 6., 12.);
        let to = point(0., 4., 0.);
        let up = vector(0., 1., 0.);

        let camera = Camera::new(
            2048,
            1080,
            consts::FRAC_PI_2,
            Transformation::new_view(&from, &to, &up),
        );

        let canvas = camera.render(world);

        let r = Rendering::new(format!("world_render_3_{}", i), canvas);
        r.save_to_file()?
    }
    Ok(())
}

pub fn world_rendering_4() -> std::io::Result<()> {
    let whiteish = Material::new_with_pattern(
        Color::from((50, 50, 70)),
        Some(Pattern::new(
            STRIPE_X_WHITE_BLACK,
            Transformation::identity(),
        )),
        0.1,
        0.3,
        0.3,
        1.,
        0.05,
        0.,
        1.,
    );
    let red_blue = Material::new_with_pattern(
        Color::from((50, 50, 50)),
        Some(Pattern::new(
            GRADIENT_X_RED_BLUE,
            Transformation::new_translation(0.5, 0., 0.).scaled(2., 1., 1.),
        )),
        0.3,
        0.6,
        0.4,
        1000.,
        0.1,
        0.,
        1.,
    );
    let world = World::new(
        vec![
            Shape::new_sphere(red_blue.clone(), Transformation::new_scaling(3., 3., 3.)),
            Shape::new_sphere(
                red_blue.clone(),
                Transformation::new_scaling(3., 3., 3.)
                    .rotated_z(consts::FRAC_PI_3)
                    .translated(4., 5., -20.),
            ),
            Shape::new_sphere(
                red_blue.clone(),
                Transformation::new_scaling(3., 3., 3.)
                    .rotated_z(consts::FRAC_PI_3 * 2.)
                    .translated(8., 10., -40.),
            ),
            Shape::new_plane(
                whiteish.clone(),
                Transformation::new_translation(0., -3., 0.),
            ),
            Shape::new_plane(
                whiteish.clone(),
                Transformation::new_translation(0., 13., 0.),
            ),
            Shape::new_plane(
                whiteish.clone(),
                Transformation::new_z_rotation(consts::FRAC_PI_2).translated(-5., 0., 0.),
            ),
        ],
        vec![PointLight::new(point(10., 10., 0.), Color::white() * 3.8)],
    );

    let from = point(12., 6., 12.);
    let to = point(0., 4., 0.);
    let up = vector(0., 1., 0.);

    let camera = Camera::new(
        2048,
        1080,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );

    let canvas = camera.render(world);
    let r = Rendering::new("world_render_4", canvas);
    r.save_to_file()
}

pub fn world_rendering_5() -> std::io::Result<()> {
    // Same as 2 but for animation
    let grey = Material::new(Color::from((50, 50, 50)), 0.3, 0.6, 0.4, 1000., 0., 0., 1.);
    let base_mat = Material::new(Color::from((70, 70, 70)), 0.3, 0.5, 0., 50., 1., 0., 1.);

    let arrow_red = Material::new(Color::red(), 0.3, 0.5, 0., 500., 0., 0., 1.);
    let arrow_green = Material::new(Color::green(), 0.3, 0.5, 0., 500., 0., 0., 1.);
    let arrow_blue = Material::new(Color::blue(), 0.3, 0.5, 0., 500., 0., 0., 1.);

    let light_setup = Transformation::new_translation(3., 12., 0.).rotated_y(consts::FRAC_PI_4);
    let light_move = Transformation::new_translation(6., 0., 6.);
    let frames = 36;
    for i in 0..frames {
        let angle = consts::PI * 2. * i as f64 / frames as f64;
        let l1 = &light_move * (&light_setup * Point::origin()).rotated_y(angle);
        let l2 = &light_move
            * (&light_setup * Point::origin()).rotated_y(consts::FRAC_PI_3 * 2.0 + angle);
        let l3 = &light_move
            * (&light_setup * Point::origin()).rotated_y(consts::FRAC_PI_3 * 4.0 + angle);

        let world = World::new(
            vec![
                Shape::new_sphere(grey.clone(), Transformation::new_translation(6., 3., 6.)),
                Shape::new(
                    SPHERE,
                    grey.clone(),
                    Transformation::new_scaling(1., 3., 1.).translated(6., 8., 6.),
                ),
                // floors and walls
                Shape::new(
                    SPHERE,
                    base_mat.clone(),
                    Transformation::new_scaling(1000., 1000., 0.01),
                ),
                Shape::new(
                    SPHERE,
                    base_mat.clone(),
                    Transformation::new_scaling(0.01, 1000., 1000.),
                ),
                Shape::new(
                    SPHERE,
                    base_mat.clone(),
                    Transformation::new_scaling(1000., 0.01, 1000.),
                ),
                // arrow thingies
                Shape::new(
                    SPHERE,
                    arrow_blue.clone(),
                    Transformation::new_scaling(1000., 0.1, 0.1),
                ),
                Shape::new(
                    SPHERE,
                    arrow_green.clone(),
                    Transformation::new_scaling(0.1, 1000., 0.1),
                ),
                Shape::new(
                    SPHERE,
                    arrow_red.clone(),
                    Transformation::new_scaling(0.1, 0.1, 1000.),
                ),
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
            2048,
            1080,
            consts::FRAC_PI_2,
            Transformation::new_view(&from, &to, &up),
        );

        let canvas = camera.render(world);

        let r = Rendering::new(format!("world_render_5_{}", i), canvas);
        r.save_to_file()?
    }
    Ok(())
}

pub fn world_rendering_6() -> std::io::Result<()> {
    let whiteish = Material::new_with_pattern(
        Color::from((50, 50, 70)),
        Some(Pattern::new(
            CHECKERS_WHITE_BLACK,
            Transformation::new_scaling(10., 10., 10.).rotated_y(1.),
        )),
        0.1,
        0.3,
        0.3,
        200.,
        0.05,
        0.,
        1.,
    );
    let blueish = Material::new_with_pattern(
        Color::from((20, 20, 100)),
        None,
        0.2,
        0.3,
        0.6,
        50.,
        0.0,
        0.,
        1.,
    );
    let glass = Material::new(
        Color::from((20, 20, 20)),
        0.1,
        0.1,
        0.8,
        2000.,
        0.9,
        1.,
        1.5,
    );
    let glass2 = Material::new(
        Color::from((20, 20, 20)),
        0.1,
        0.1,
        0.8,
        2000.,
        0.9,
        1.,
        4.5,
    );
    let mirror = Material::new(Color::from((1, 1, 1)), 0.2, 0.8, 0.8, 200., 0.9, 0., 1.);
    let world = World::new(
        vec![
            Shape::new_sphere(
                glass.clone(),
                Transformation::new_scaling(3., 3., 3.).translated(0., 1., 0.),
            ),
            Shape::new_sphere(glass2.clone(), Transformation::new_translation(0., 1., 0.)),
            /*
            Shape::new_sphere(
                glass.clone(),
                Transformation::new_scaling(3., 3., 3.)
                    .rotated_z(consts::FRAC_PI_3)
                    .translated(4., 5., -20.),
            ),
            */
            Shape::new_sphere(
                mirror.clone(),
                Transformation::new_translation(-7., 1., 4.).scaled(5., 5., 5.),
            ),
            /*
            Shape::new_plane(
                mirror.clone(),
                Transformation::new_x_rotation(consts::FRAC_PI_3).translated(0., 0., -5.),
            ),
            */
            Shape::new_plane(
                whiteish.clone(),
                Transformation::new_translation(0., -3., 0.),
            ),
            Shape::new_plane(
                blueish.clone(),
                Transformation::new_translation(0., 10., 0.),
            ),
        ],
        vec![PointLight::new(point(10., 10., 0.), Color::white() * 3.8)],
    );

    let from = point(20., 10., 5.);
    let to = point(0., 0., 0.);
    let up = vector(0., 1., 0.);

    let camera = Camera::new(
        3840,
        2160,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );

    let canvas = camera.render(world);
    let r = Rendering::new("world_render_6", canvas);
    r.save_to_file()
}

pub fn world_rendering_7() -> std::io::Result<()> {
    let whiteish = Material::new_with_pattern(
        Color::from((50, 50, 70)),
        Some(Pattern::new(
            CHECKERS_WHITE_BLACK,
            Transformation::new_scaling(1., 1., 1.).rotated_y(0.),
        )),
        0.1,
        0.3,
        0.3,
        200.,
        0.05,
        0.,
        1.,
    );
    let glass = Material::new(
        Color::from((20, 20, 20)),
        0.1,
        0.1,
        0.8,
        2000.,
        0.9,
        0.99,
        0.9,
    );
    let glass2 = Material::new(
        Color::from((20, 20, 20)),
        0.1,
        0.1,
        0.8,
        2000.,
        0.9,
        1.,
        1.2,
    );
    let world = World::new(
        vec![
            Shape::new_sphere(
                glass.clone(),
                Transformation::new_scaling(3., 3., 3.).translated(0., 3., 0.),
            ),
            Shape::new_sphere(glass2.clone(), Transformation::new_translation(0., 3., 0.)),
            Shape::new_plane(
                whiteish.clone(),
                Transformation::new_translation(0., -5., 0.),
            ),
        ],
        vec![PointLight::new(point(0., 20., 0.), Color::white() * 3.8)],
    );

    let from = point(5., 10., 5.);
    let to = point(0., 0., 0.);
    let up = vector(0., 1., 0.);

    let camera = Camera::new(
        10000,
        5000,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );

    let canvas = camera.render(world);
    let r = Rendering::new("world_render_7", canvas);
    r.save_to_file()
}
