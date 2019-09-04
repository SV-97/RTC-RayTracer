use std::sync::Arc;

use super::prelude::*;

use crate::{shading::Material, primitives::{
    ray::Ray,
    vector::{Point, ScalarProd, Transformation, Vec3D},
}};

pub static SPHERE: ShapeFuncs = (intersect, normal_at);

fn intersect(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    let inverse = shape.inverse_transform();
    let ray2 = ray.transform(inverse);
    let sphere_to_ray = (&ray2.origin) - Point::origin();
    let a = (&ray2.direction).scalar_prod(&ray2.direction);
    let b = 2.0 * (&ray2.direction).scalar_prod(&sphere_to_ray);
    let c = (&sphere_to_ray).scalar_prod(&sphere_to_ray) - 1.0;
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        let d_sqrt = discriminant.sqrt();
        let ta = 2.0 * a;
        let t1 = (-b - d_sqrt) / ta;
        let t2 = (-b + d_sqrt) / ta;
        let v = vec![t1, t2];
        Some(Intersections::new(
            v.into_iter()
                .map(|t| Intersection::new(t, shape.clone()))
                .collect::<Vec<_>>(),
        ))
    }
}

fn normal_at(shape: Arc<Shape>, point: &Point) -> Vec3D {
    let inverse = shape.inverse_transform();
    let object_point = inverse * point;
    let world_transform = inverse.transpose();
    let object_normal = object_point - Point::origin();
    let mut out = world_transform * object_normal;
    out.set_w(0.0);
    out.unit()
}

impl Default for Shape {
    fn default() -> Self {
        Self::new(Transformation::identity(), Material::default(), SPHERE)
    }
}