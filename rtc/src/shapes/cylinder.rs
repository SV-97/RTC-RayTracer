use std::sync::Arc;

use super::prelude::*;

use crate::{
    primitives::{
        approx_eq::ApproxEq,
        ray::Ray,
        vector::{vector, Point, Transformation, Vec3D},
    },
    shading::Material,
};

pub static CYLINDER: ShapeFuncs = (intersect, normal_at);
pub static CYLINDER_TRUNC: ShapeFuncs = (intersect_trunc, normal_at);

fn intersect(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    base_shape_intersect(shape, ray, |shape, ray| {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);
        if a.approx_eq(0.0) {
            None
        } else {
            let b = 2.0 * (ray.origin.x() * ray.direction.x() + ray.origin.z() * ray.direction.z());
            let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                None
            } else {
                let t0 = (-b - disc.sqrt()) / (2.0 * a);
                let t1 = (-b + disc.sqrt()) / (2.0 * a);
                let xs = Intersections::new(vec![
                    Intersection::new(t0, Arc::clone(&shape)),
                    Intersection::new(t1, Arc::clone(&shape)),
                ]);
                Some(xs)
            }
        }
    })
}

fn normal_at(shape: Arc<Shape>, point: &Point) -> Vec3D {
    base_shape_normal(shape, point, |_, point| vector(point.x(), 0., point.z()))
}

fn intersect_trunc(shape: Arc<Shape>, ray: &Ray) -> Option<Intersections> {
    base_shape_intersect(shape, ray, |shape, ray| {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);
        if a.approx_eq(0.0) {
            None
        } else {
            let b = 2.0 * (ray.origin.x() * ray.direction.x() + ray.origin.z() * ray.direction.z());
            let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                None
            } else {
                let t0 = (-b - disc.sqrt()) / (2.0 * a);
                let t1 = (-b + disc.sqrt()) / (2.0 * a);

                let mut xs = vec![];
                let y0 = ray.origin.y() + t0 * ray.direction.y();
                if -1.0 < y0 && y0 < 1.0 {
                    xs.push(Intersection::new(t0, Arc::clone(&shape)));
                }
                let y1 = ray.origin.y() + t1 * ray.direction.y();
                if -1.0 < y1 && y1 < 1.0 {
                    xs.push(Intersection::new(t1, Arc::clone(&shape)));
                }
                Some(Intersections::new(xs))
            }
        }
    })
}

impl Shape {
    pub fn new_cylinder(material: Material, transform: Transformation) -> Self {
        Self::new(CYLINDER, material, transform)
    }

    pub fn default_cylinder() -> Self {
        Self::new_cylinder(Material::default(), Transformation::identity())
    }

    pub fn new_trunc_cylinder(material: Material, transform: Transformation) -> Self {
        Self::new(CYLINDER, material, transform)
    }

    pub fn default_trunc_cylinder() -> Self {
        Self::new_cylinder(Material::default(), Transformation::identity())
    }
}
