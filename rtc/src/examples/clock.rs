use crate::{
    primitives::{canvas::Canvas, pixel::Pixel, rendering::Rendering, vector::point},
    utils::typelevel_nums::*,
};
use std::f64::consts;

/// Draws the dots of a clockface
pub fn draw_clockface() -> std::io::Result<()> {
    let mut canvas = Canvas::<N900, N900>::new();
    let pen = Pixel::from((20, 150, 255)); // blue
    let origin = point(canvas.width() as f64 / 2., canvas.height() as f64 / 2., 0.);
    let top = point(0., origin.y() / 2., 0.);
    for i in 0..12 {
        let pos = top
            .rotate_z(i as f64 * consts::PI / 6.)
            .translate(origin.x(), origin.y(), 0.);
        let _ = canvas
            .draw_block(
                pos.x().round() as usize,
                pos.y().round() as usize,
                2,
                2,
                pen,
            )
            .iter()
            .map(|s| println!("{}", s));
    }
    let r = Rendering::new("clock", canvas);
    r.save_to_file()
}
