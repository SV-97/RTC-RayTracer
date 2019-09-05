use std::{fmt, sync::Arc};

use crate::{
    primitives::{
        approx_eq::ApproxEq,
        vector::{Point, Transformation},
    },
    shapes::Shape,
};

use super::Color;

pub type PatternFunc = fn(&Point) -> Color;

#[derive(Clone)]
pub struct Pattern {
    transform: Transformation,
    inverse_transform: Transformation,
    pattern_function: PatternFunc,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pattern {{ transformation: {:?}, inverse_transformation: {:?}}}",
            self.transform, self.inverse_transform
        )
    }
}

impl Pattern {
    pub fn new(pattern_function: PatternFunc, transform: Transformation) -> Self {
        let inverse_transform = transform
            .invert()
            .expect("Encountered non invertible matrix.");
        Pattern {
            transform,
            inverse_transform,
            pattern_function,
        }
    }

    /// Find the color of the pattern given a point and the associated object
    /// We first transform the point to object space and then to pattern space
    pub fn at(&self, object: Arc<Shape>, point: &Point) -> Color {
        let object_point = object.inverse_transform() * point;
        let pattern_point = &self.inverse_transform * object_point;
        (self.pattern_function)(&pattern_point)
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &Transformation {
        &self.inverse_transform
    }

    pub fn set_transform(&mut self, transformation: Transformation) {
        self.transform = transformation;
        self.inverse_transform = self.transform.invert().unwrap();
    }

    pub fn modify_transform(&mut self, f: impl Fn(&mut Transformation) -> &mut Transformation) {
        f(&mut self.transform);
        self.inverse_transform = self.transform.invert().unwrap();
    }
}

impl ApproxEq for &Pattern {
    fn approx_eq(self, other: Self) -> bool {
        self.transform.approx_eq(&other.transform)
    }
}
