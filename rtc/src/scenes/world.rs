use std::sync::Arc;

use crate::{
    primitives::{
        ray::Ray,
        vector::{point, Point, Transformation},
    },
    shading::{Color, Material, PointLight},
    shapes::{Intersections, PreComp, Shape, SPHERE},
};

pub struct World {
    pub objects: Vec<Arc<Shape>>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Shape>, lights: Vec<PointLight>) -> Self {
        let objects = objects.into_iter().map(Arc::new).collect::<Vec<_>>();
        World { objects, lights }
    }

    pub fn new_empty() -> Self {
        Self::new(vec![], vec![])
    }

    /// Find the intersections of a ray with all objects in the scene sorted by their t-value
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut v = self
            .objects
            .iter()
            .map(|shape| (shape.intersect)(Arc::clone(shape), ray))
            .filter_map(|x| x.map(Intersections::into_iter))
            .flatten()
            .collect::<Vec<_>>();
        v.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections::new(v)
    }

    /// Cast a shadow ray to determine whether a point is in shadow
    pub fn is_shadowed(&self, point: &Point) -> Vec<bool> {
        self.lights
            .iter()
            .map(|light| {
                let v = &light.position - point;
                let distance = v.mag();
                let direction = v.unit();
                let ray = Ray::new((*point).clone(), direction.clone());
                let is = self.intersect(&ray);
                let h = is.hit();
                match h {
                    Some(hit) => hit.t < distance,
                    None => false,
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn shade_hit(&self, comp: &PreComp) -> Color {
        self.lights
            .iter()
            .zip(self.is_shadowed(&comp.over_point).into_iter())
            .map(|(light, shadowed)| {
                light.lighting(
                    &comp.object.material,
                    &comp.point,
                    &comp.eye,
                    &comp.normal,
                    shadowed,
                )
            })
            .fold(None, |blend: Option<Color>, new_color| match blend {
                Some(blend) => Some(new_color.blend_lighten_only(blend)),
                None => Some(new_color),
            })
            .unwrap()
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let is = self.intersect(ray);
        match is.hit() {
            Some(hit) => {
                let precomp = hit.clone().prepare_computations(ray);
                self.shade_hit(&precomp)
            }
            None => Color::black(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(point(-10., 10., -10.), Color::new_rgb(1., 1., 1.));
        let s1 = Shape::new(
            Transformation::identity(),
            Material::new(Color::new_rgb(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0),
            SPHERE,
        );
        let mut s2 = Shape::default();
        s2.modify_transform(|t| {
            t.scale(0.5, 0.5, 0.5);
        });
        World::new(vec![s1, s2], vec![light])
    }
}
