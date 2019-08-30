use crate::{
    primitives::{
        ray::Ray,
        vector::{point, Transformation},
    },
    shading::{Color, Material, PointLight},
    shapes::{Intersection, Intersections, PreComp, Shape, Sphere},
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight,
}

impl World {
    pub fn new(objects: Vec<Sphere>, light: PointLight) -> Self {
        World { objects, light }
    }

    pub fn new_empty() -> Self {
        Self::new(vec![], PointLight::default())
    }

    /// Find the intersections of a ray with all objects in the scene sorted by their t-value
    pub fn intersect<'a>(&'a self, ray: &'a Ray) -> Vec<Intersection<'a, Sphere>> {
        let mut v = self
            .objects
            .iter()
            .map(|s| s.intersect(ray))
            .filter_map(|x| x.map(Intersections::into_iter))
            .flatten()
            .collect::<Vec<_>>();
        v.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        v
    }

    pub fn shade_hit<'a, T: Shape<'a>>(&self, comp: &PreComp<'a, T>) -> Color {
        self.light
            .lighting(comp.object.material(), &comp.point, &comp.eye, &comp.normal)
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(point(-10., 10., -10.), Color::new_rgb(1., 1., 1.));
        let s1 = Sphere::new(
            Material::new(Color::new_rgb(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0),
            Transformation::identity(),
        );
        let mut s2 = Sphere::default();
        s2.get_transform_mut().scale(0.5, 0.5, 0.5);
        World::new(vec![s1, s2], light)
    }
}
