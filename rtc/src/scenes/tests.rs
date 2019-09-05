use super::*;

use crate::{
    assert_approx_eq,
    primitives::{
        approx_eq::EPSILON_F64,
        ray::Ray,
        vector::{point, vector, Point, Transformation},
    },
    shading::{Color, Material, PointLight},
    shapes::{Intersection, Intersections, Shape, SPHERE},
};

use std::{f64::consts, sync::Arc};

#[test]
fn intersect() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = w.intersect(&r);
    assert_eq!(xs.len(), 4);
    assert_approx_eq!(xs[0].t, 4.0);
    assert_approx_eq!(xs[1].t, 4.5);
    assert_approx_eq!(xs[2].t, 5.5);
    assert_approx_eq!(xs[3].t, 6.0);
}

#[test]
fn shade_intersection() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let s = &w.objects[0];
    let i = Intersection::new(4., Arc::clone(s));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let c = w.shade_hit(&comps, 1);
    assert_approx_eq!(c, Color::new_rgb(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_intersection_inside() {
    let mut w = World::default();
    w.lights = vec![PointLight::new(
        point(0., 0.25, 0.),
        Color::new_rgb(1., 1., 1.),
    )];
    let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
    let s = &w.objects[1];
    let i = Intersection::new(0.5, Arc::clone(s));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let c = w.shade_hit(&comps, 1);
    assert_approx_eq!(c, Color::new_rgb(0.90498, 0.90498, 0.90498));
}

#[test]
fn color_at_miss() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
    let c = w.color_at(&r, 1);
    assert_approx_eq!(c, Color::new_rgb(0., 0., 0.));
}

#[test]
fn color_at_hit() {
    let w = World::default();
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let c = w.color_at(&r, 1);
    assert_approx_eq!(c, Color::new_rgb(0.38066, 0.47583, 0.2855));
}

#[test]
fn color_at_from_inside() {
    let mut w = World::default();
    let outer = Arc::make_mut(&mut w.objects[0]);
    outer.material.ambient = 1.0;
    drop(outer);
    let inner = Arc::make_mut(&mut w.objects[1]);
    inner.material.ambient = 1.0;
    let inner = &w.objects[1];

    let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
    let c = w.color_at(&r, 1);
    assert_approx_eq!(c, inner.material.color);
}

#[test]
fn camera_new() {
    let cam = Camera::new(160, 120, consts::FRAC_PI_2, Transformation::identity());
    assert_eq!(cam.height(), 120);
    assert_eq!(cam.width(), 160);
    assert_eq!(cam.fov(), consts::FRAC_PI_2);
    assert_approx_eq!(cam.transform(), &Transformation::identity());
}

#[test]
fn camera_pixel_size() {
    let cam = Camera::new(200, 125, consts::FRAC_PI_2, Transformation::identity());
    assert_eq!(cam.height(), 125);
    assert_eq!(cam.width(), 200);
    assert_eq!(cam.fov(), consts::FRAC_PI_2);
    assert_approx_eq!(cam.transform(), &Transformation::identity());
}

#[test]
fn cons_ray_center() {
    let cam = Camera::new(201, 101, consts::FRAC_PI_2, Transformation::identity());
    let ray = cam.ray_for_pixel(100, 50);
    assert_approx_eq!(ray.origin, &point(0., 0., 0.));
    assert_approx_eq!(ray.direction, &vector(0., 0., -1.));
}

#[test]
fn cons_ray_corner() {
    let cam = Camera::new(201, 101, consts::FRAC_PI_2, Transformation::identity());
    let ray = cam.ray_for_pixel(0, 0);
    assert_approx_eq!(ray.origin, &point(0., 0., 0.));
    assert_approx_eq!(ray.direction, &vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn cons_ray_transformed_cam() {
    let cam = Camera::new(
        201,
        101,
        consts::FRAC_PI_2,
        Transformation::new_translation(0., -2., 5.).rotated_y(consts::FRAC_PI_4),
    );
    let ray = cam.ray_for_pixel(100, 50);
    let a = consts::SQRT_2 / 2.0;
    assert_approx_eq!(ray.origin, &point(0., 2., -5.));
    assert_approx_eq!(ray.direction, &vector(a, 0., -a));
}

#[test]
fn render_default_with_cam() {
    let from = point(0., 0., -5.);
    let to = Point::origin();
    let up = vector(0., 1., 0.);
    let cam = Camera::new(
        11,
        11,
        consts::FRAC_PI_2,
        Transformation::new_view(&from, &to, &up),
    );
    let image = cam.render(World::default());
    assert_approx_eq!(image[(5, 5)], Color::new_rgb(0.38066, 0.47583, 0.2855));
}

#[test]
fn shadow_nothing_colinear() {
    let w = World::default();
    let p = point(0., 10., 0.);
    assert!(!w.is_shadowed(&p)[0]);
}

#[test]
fn shadow_object_in_path() {
    let w = World::default();
    let p = point(10., -10., 10.);
    assert!(w.is_shadowed(&p)[0]);
}

#[test]
fn shadow_point_behind() {
    let w = World::default();
    let p = point(-20., 20., -20.);
    assert!(!w.is_shadowed(&p)[0]);
}

#[test]
fn shadow_object_behind() {
    let w = World::default();
    let p = point(-2., 2., -2.);
    assert!(!w.is_shadowed(&p)[0]);
}

#[test]
fn shade_hit_intersection_in_shadow() {
    let s2 = Shape::new(
        SPHERE,
        Material::default(),
        Transformation::new_translation(0., 0., 10.),
    );
    let w = World::new(
        vec![Shape::default(), s2.clone()],
        vec![PointLight::new(
            point(0., 0., -10.),
            Color::new_rgb(1., 1., 1.),
        )],
    );
    let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
    let i = Intersection::new(4., Arc::new(s2));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let c = w.shade_hit(&comps, 1);
    assert_approx_eq!(c, Color::new_rgb(0.1, 0.1, 0.1));
}

#[test]
fn shadow_hit_offset_point() {
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let shape = Shape::new(
        SPHERE,
        Material::default(),
        Transformation::new_translation(0., 0., 1.),
    );
    let shape = Arc::new(shape);
    let i = Intersection::new(5., shape);
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    assert!(comps.over_point.z() < -EPSILON_F64 / 2.0);
    assert!(comps.point.z() > comps.over_point.z());
}

#[test]
fn nonreflective_reflection() {
    let light = PointLight::new(point(-10., 10., -10.), Color::new_rgb(1., 1., 1.));
    let s1 = Shape::new(
        SPHERE,
        Material::new(
            Color::new_rgb(0.8, 1.0, 0.6),
            0.1,
            0.7,
            0.2,
            200.0,
            0.,
            0.,
            1.,
        ),
        Transformation::identity(),
    );
    let mut s2 = Shape::default();
    s2.modify_transform(|t| t.scale(0.5, 0.5, 0.5));
    s2.material.ambient = 1.0;
    let w = World::new(vec![s1, s2], vec![light]);
    let r = Ray::new(Point::origin(), vector(0., 0., 1.));
    let shape = &w.objects[1];
    let i = Intersection::new(1., Arc::clone(shape));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let color = w.reflected_color(&comps, 1);
    assert_approx_eq!(color, Color::black());
}

#[test]
fn reflective_reflection() {
    let mut w = World::default();
    let mut mat = Material::default();
    mat.reflectiveness = 0.5;
    let shape = Arc::new(Shape::new_plane(
        mat,
        Transformation::new_translation(0., -1., 0.),
    ));
    w.objects.push(Arc::clone(&shape));
    let a = consts::SQRT_2 / 2.0;
    let r = Ray::new(point(0., 0., -3.), vector(0., -a, a));
    let i = Intersection::new(consts::SQRT_2, Arc::clone(&shape));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let color = w.reflected_color(&comps, 1);
    // assert_approx_eq!(color, Color::new_rgb(0.190347, 0.23793, 0.14276));
    assert_approx_eq!(color, Color::new_rgb(0.19032, 0.2379, 0.14274));
}

#[test]
fn shade_hit_with_reflective_mat() {
    let mut w = World::default();
    let mut mat = Material::default();
    mat.reflectiveness = 0.5;
    let shape = Arc::new(Shape::new_plane(
        mat,
        Transformation::new_translation(0., -1., 0.),
    ));
    w.objects.push(Arc::clone(&shape));
    let a = consts::SQRT_2 / 2.0;
    let r = Ray::new(point(0., 0., -3.), vector(0., -a, a));
    let i = Intersection::new(consts::SQRT_2, Arc::clone(&shape));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let color = w.shade_hit(&comps, 1);
    assert_approx_eq!(color, Color::new_rgb(0.87677, 0.92436, 0.82918));
}

#[test]
fn prevent_infinite_reflection() {
    let mut mat = Material::default();
    mat.reflectiveness = 1.0;
    let world = World::new(
        vec![
            Shape::new_plane(mat.clone(), Transformation::new_translation(0., -1., 0.)),
            Shape::new_plane(mat.clone(), Transformation::new_translation(0., 1., 0.)),
        ],
        vec![PointLight::new(Point::origin(), Color::white())],
    );
    let r = Ray::new(point(0., 0., 0.), vector(0., 1., 0.));
    world.color_at(&r, 100);
}

#[test]
fn reflection_at_maximum_recursion() {
    let mut w = World::default();
    let mut mat = Material::default();
    mat.reflectiveness = 0.5;
    let shape = Arc::new(Shape::new_plane(
        mat,
        Transformation::new_translation(0., -1., 0.),
    ));
    w.objects.push(Arc::clone(&shape));
    let a = consts::SQRT_2 / 2.0;
    let r = Ray::new(point(0., 0., -3.), vector(0., -a, a));
    let i = Intersection::new(consts::SQRT_2, Arc::clone(&shape));
    let is = Intersections::new(vec![i]);
    let comps = is[0].prepare_computations(&r, &is);
    let color = w.reflected_color(&comps, 0);
    assert_approx_eq!(color, Color::new_rgb(0., 0., 0.));
}

#[test]
fn refactored_opaque() {
    let w = World::default();
    let shape = &w.objects[0];
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = Intersections::new(vec![
        Intersection::new(4., Arc::clone(shape)),
        Intersection::new(6., Arc::clone(shape)),
    ]);
    let comps = xs[0].prepare_computations(&r, &xs);
    let c = w.refracted_color(&comps, 5);
    assert_approx_eq!(c, Color::black());
}

#[test]
fn refactored_at_max_depth() {
    let w = {
        let light = PointLight::new(point(-10., 10., -10.), Color::new_rgb(1., 1., 1.));
        let s1 = Shape::new(
            SPHERE,
            Material::new(
                Color::new_rgb(0.8, 1.0, 0.6),
                0.1,
                0.7,
                0.2,
                200.0,
                0.,
                1.,
                1.5,
            ),
            Transformation::identity(),
        );
        let mut s2 = Shape::default();
        s2.modify_transform(|t| t.scale(0.5, 0.5, 0.5));
        World::new(vec![s1, s2], vec![light])
    };
    let shape = &w.objects[0];
    let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
    let xs = Intersections::new(vec![
        Intersection::new(4., Arc::clone(shape)),
        Intersection::new(6., Arc::clone(shape)),
    ]);
    let comps = xs[0].prepare_computations(&r, &xs);
    let c = w.refracted_color(&comps, 0);
    assert_approx_eq!(c, Color::black());
}

#[test]
fn total_internal_reflection() {
    let w = {
        let light = PointLight::new(point(-10., 10., -10.), Color::new_rgb(1., 1., 1.));
        let s1 = Shape::new(
            SPHERE,
            Material::new(
                Color::new_rgb(1., 1.0, 1.),
                0.1,
                0.9,
                0.9,
                200.0,
                0.,
                1.,
                1.5,
            ),
            Transformation::identity(),
        );
        let mut s2 = Shape::default();
        s2.modify_transform(|t| t.scale(0.5, 0.5, 0.5));
        World::new(vec![s1, s2], vec![light])
    };
    let shape = &w.objects[0];
    let a = consts::SQRT_2 / 2.0;
    let r = Ray::new(point(0., 0., a), vector(0., 1., 0.));
    let xs = Intersections::new(vec![
        Intersection::new(-a, Arc::clone(shape)),
        Intersection::new(a, Arc::clone(shape)),
    ]);
    let comps = xs[1].prepare_computations(&r, &xs);
    let c = w.refracted_color(&comps, 5);
    assert_approx_eq!(c, Color::black());
}
