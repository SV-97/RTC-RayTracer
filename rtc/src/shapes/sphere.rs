use std::cell::{Ref, RefCell};

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
    inverse_transformation: RefCell<Option<Transformation>>,
    pub material: Material,
}

impl Sphere {
    pub fn new(material: Material, transformation: Transformation) -> Self {
        Self {
            material,
            transformation,
            inverse_transformation: RefCell::new(None),
        }

    }

    /// Calculate the normal vector for any point on the sphere
    pub fn normal_at(&self, point: &Point) -> Vec3D {
        let mut object_point = None;
        let mut world_transform = None;
        Ref::map(self.get_inverse_transform(), |inverse| {
            object_point = Some(inverse * point);
            world_transform = Some(inverse.transpose());
            inverse
        });
        if let (Some(object_point), Some(world_transform)) = (object_point, world_transform) {
            let object_normal = object_point - Point::origin();
            let mut out = world_transform * object_normal;
            out.set_w(0.0);
            out.unit()
        } else {
            unreachable!()
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transformation: Transformation::identity(),
            inverse_transformation: RefCell::new(Some(Transformation::identity())),
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
        let mut out = None;
        Ref::map(self.get_inverse_transform(), |inverse| {
            let ray2 = ray.transform(inverse);
            let sphere_to_ray = (&ray2.origin) - Point::origin();
            let a = (&ray2.direction).scalar_prod(&ray2.direction);
            let b = 2.0 * (&ray2.direction).scalar_prod(&sphere_to_ray);
            let c = (&sphere_to_ray).scalar_prod(&sphere_to_ray) - 1.0;
            let discriminant = b.powi(2) - 4.0 * a * c;
            if discriminant < 0.0 {
                out = None;
            } else {
                let d_sqrt = discriminant.sqrt();
                let ta = 2.0 * a;
                let t1 = (-b - d_sqrt) / ta;
                let t2 = (-b + d_sqrt) / ta;
                let v = vec![t1, t2];
                out = Some(Intersections::new(
                    v.into_iter()
                        .map(|t| Intersection::new(t, self))
                        .collect::<Vec<_>>(),
                ));
            }
            inverse
        });
        out
    }

    fn set_transform(&mut self, transformation: Transformation) {
        self.transformation = transformation;
        self.inverse_transformation.replace(None);
    }

    fn get_transform(&'a self) -> &'a Transformation {
        &self.transformation
    }

    fn get_inverse_transform(&'a self) -> Ref<'a, Transformation> {
        self.inverse_transformation
            .borrow_mut()
            .get_or_insert_with(|| {
                self.transformation
                    .invert()
                    .expect("Encountered uninvertable matrix")
            });
        Ref::map(self.inverse_transformation.borrow(), |opt_ref| {
            opt_ref.as_ref().unwrap()
        })
    }
    fn get_transform_mut(&'a mut self) -> &'a mut Transformation {
        self.inverse_transformation.replace(None);
        &mut self.transformation
    }
}
