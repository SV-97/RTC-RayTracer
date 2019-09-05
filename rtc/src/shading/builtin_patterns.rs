use crate::primitives::vector::Point;

use super::{Color, PatternFunc};

// FIXME The functions are aliased to constants because this system is supposed
// to be used with const fns once they're stable. That'll enable higher order functions
// like stripe_x that then produce the real pattern function (at least I hope so).
// One could add a macro for all basic patterns that then generate specialized
// functions... but eh.
pub static STRIPE_X_WHITE_BLACK: PatternFunc = stripe_x_white_black;
pub static GRADIENT_X_WHITE_BLACK: PatternFunc = gradient_x_white_black;
pub static GRADIENT_X_RED_BLUE: PatternFunc = gradient_x_red_blue;
pub static RING_XZ_WHITE_BLACK: PatternFunc = ring_xz_white_black;
pub static CHECKERS_WHITE_BLACK: PatternFunc = checkers_white_black;

fn stripe_x_white_black(point: &Point) -> Color {
    let color_1 = Color::white();
    let color_2 = Color::black();
    if point.x().floor() % 2.0 == 0.0 {
        color_1
    } else {
        color_2
    }
}
/*
macro_rules! gradient {
    ($color_1:expr, $color_2:expr, $name:ident) => {
    };
}
*/

fn gradient_x_white_black(point: &Point) -> Color {
    let color_1 = Color::white();
    let color_2 = Color::black();
    color_1 + (color_2 - color_1) * (point.x() - point.x().floor()) as f32
}

fn gradient_x_red_blue(point: &Point) -> Color {
    let color_1 = Color::red();
    let color_2 = Color::blue();
    color_1 + (color_2 - color_1) * (point.x() - point.x().floor()) as f32
}

fn ring_xz_white_black(point: &Point) -> Color {
    let color_1 = Color::white();
    let color_2 = Color::black();
    let x = point.x();
    let z = point.z();
    if (x * x + z * z).sqrt().floor() % 2.0 == 0.0 {
        color_1
    } else {
        color_2
    }
}

fn checkers_white_black(point: &Point) -> Color {
    let color_1 = Color::white();
    let color_2 = Color::black();
    let x = point.x();
    let y = point.y();
    let z = point.z();
    if (x.floor() + y.floor() + z.floor()) % 2.0 == 0.0 {
        color_1
    } else {
        color_2
    }
}
