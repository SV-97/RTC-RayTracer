use super::prelude::*;

use crate::primitives::{
    ray::Ray,
    vector::{Point, ScalarProd, Vec3D},
};

impl Shape {
    pub fn new(transform: Transformation, material: Material, shape_type: ShapeType) -> Self {
        let inverse_transform = transform
            .invert()
            .expect("Encountered non invertible matrix.");
        Shape {
            transform,
            inverse_transform,
            material,
            shape_type,
        }
    }

    pub fn transform<'a>(&'a self) -> &'a Transformation {
        &self.transform
    }

    pub fn inverse_transform<'a>(&'a self) -> &'a Transformation {
        &self.inverse_transform
    }

    pub fn set_transform(&mut self, transformation: Transformation) {
        self.transform = transformation;
        self.inverse_transform = self.transform.invert().unwrap();
    }

    pub fn modify_transform(&mut self, f: impl Fn(&mut Transformation)) {
        f(&mut self.transform);
        self.inverse_transform = self.transform.invert().unwrap();
    }
}

impl Shape {
    pub fn intersect(&'a self, ray: &Ray) -> Option<Intersections> {
        match self.shape_type {
            ShapeType::Sphere => {
                let inverse = self.inverse_transform();
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
                            .map(|t| Intersection::new(t, self))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
        }
    }

    pub fn normal_at(&self, point: &Point) -> Vec3D {
        match self.shape_type {
            ShapeType::Sphere => {
                let inverse = self.inverse_transform();
                let object_point = inverse * point;
                let world_transform = inverse.transpose();
                let object_normal = object_point - Point::origin();
                let mut out = world_transform * object_normal;
                out.set_w(0.0);
                out.unit()
            }
        }
    }
}
