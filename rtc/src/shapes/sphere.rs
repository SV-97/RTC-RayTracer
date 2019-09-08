use std::sync::Arc;

use super::prelude::*;

use crate::{
    primitives::{
        ray::Ray,
        vector::{Point, ScalarProd, Transformation, Vec3D},
    },
    shading::Material,
};

pub static SPHERE: ShapeFuncs = (intersect, normal_at);

fn intersect(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    base_shape_intersect(shape, ray, |shape, _| {
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
    })
}

fn normal_at(shape: Arc<Shape>, point: &Point) -> Vec3D {
    base_shape_normal(shape, point, |_, object_point| {
        object_point - Point::origin()
    })
}

impl Shape {
    pub fn new_sphere(material: Material, transform: Transformation) -> Self {
        Self::new(SPHERE, material, transform)
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::new(SPHERE, Material::default(), Transformation::identity())
    }
}
