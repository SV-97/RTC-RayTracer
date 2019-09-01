use crate::{
    primitives::{
        approx_eq::ApproxEq,
        ray::Ray,
        vector::{Point, ScalarProd, Transformation, Vec3D},
    },
    shading::Material,
};

use super::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sphere {
    transformation: Transformation,
    inverse_transformation: Transformation,
    material: Material,
}

impl Sphere {
    pub fn new(material: Material, transformation: Transformation) -> Self {
        let inverse_transformation = transformation.invert().unwrap();
        Self {
            material,
            transformation,
            inverse_transformation,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transformation: Transformation::identity(),
            inverse_transformation: Transformation::identity(),
            material: Material::default(),
        }
    }
}

impl ApproxEq for Sphere {
    fn approx_eq(self, other: Self) -> bool {
        self.transformation.approx_eq(&other.transformation)
    }
}

impl ApproxEq for &Sphere {
    fn approx_eq(self, other: Self) -> bool {
        self.transformation.approx_eq(&other.transformation)
    }
}

impl<'a> Shape<'a> for Sphere {
    fn intersect(&'a self, ray: &Ray) -> Option<Intersections<'a, Self>> {
        let inverse = self.get_inverse_transform();
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

    fn set_transform(&mut self, transformation: Transformation) {
        self.transformation = transformation;
        self.inverse_transformation = self.transformation.invert().unwrap();
    }

    fn get_transform(&'a self) -> &'a Transformation {
        &self.transformation
    }

    fn get_inverse_transform(&'a self) -> &'a Transformation {
        &self.inverse_transformation
    }

    fn get_transform_mut(&'a mut self, f: impl Fn(&mut Transformation)) {
        f(&mut self.transformation);
        self.inverse_transformation = self.transformation.invert().unwrap();
    }

    /// Calculate the normal vector for any point on the sphere
    fn normal_at(&self, point: &Point) -> Vec3D {
        let inverse = self.get_inverse_transform();
        let object_point = inverse * point;
        let world_transform = inverse.transpose();
        let object_normal = object_point - Point::origin();
        let mut out = world_transform * object_normal;
        out.set_w(0.0);
        out.unit()
    }

    fn material(&'a self) -> &'a Material {
        &self.material
    }

    fn material_mut(&'a mut self) -> &'a mut Material {
        &mut self.material
    }
}
