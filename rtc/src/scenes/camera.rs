use std::{
    sync::{mpsc::sync_channel, Arc, RwLock},
    thread::spawn,
};

use crate::primitives::{
    canvas::Canvas,
    ray::Ray,
    vector::{point, Point, Transformation},
};

static MAXIMUM_REFLECTION_RECURSION_DEPTH: usize = 5;

use super::World;

/// Virtual camera
/// Virtual canvas is one unit in front of camera
#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    height: usize,
    width: usize,
    /// Field of view in rad
    fov: f64,
    /// Orientation of world relative to camera described as transformation matrix
    transform: Transformation,
    /// Inverse of transform
    inverse_transform: Transformation,
    /// World space size of pixels of canvas
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f64, transform: Transformation) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = width as f64 / height as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / width as f64;
        let inverse_transform = transform.invert().unwrap();
        Camera {
            width,
            height,
            fov,
            transform,
            pixel_size,
            half_width,
            half_height,
            inverse_transform,
        }
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    fn set_pixel_size(&mut self) {
        let half_view = (self.fov / 2.0).tan();
        let aspect = self.aspect_ratio();
        if aspect >= 1.0 {
            self.half_width = half_view;
            self.half_height = half_view / aspect;
        } else {
            self.half_width = half_view * aspect;
            self.half_height = half_view;
        }
        self.pixel_size = (self.half_width * 2.0) / self.width() as f64;
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    /// Calculate a ray through the coordinate pair (x, y) from the camera through the canvas
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let inverse = &self.inverse_transform;
        let pixel = inverse * point(world_x, world_y, -1.);
        let origin = inverse * Point::origin();
        let direction = (pixel - &origin).unit();
        Ray::new(origin, direction)
    }

    pub fn render(self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);
        let n_threads = 8;
        let (tx, rx) = sync_channel(20);
        let ys = (0..self.height()).collect::<Vec<_>>();
        let chunk_size = ys.len() / n_threads;
        let chunks = ys.chunks(chunk_size);
        let width = self.width();
        let locked_cam = Arc::new(RwLock::new(self));
        let locked_world = Arc::new(RwLock::new(world));
        for chunk in chunks {
            let t_tx = tx.clone();
            let chunk = chunk.iter().copied().collect::<Vec<usize>>();
            let t_cam = Arc::clone(&locked_cam);
            let t_world = Arc::clone(&locked_world);
            spawn(move || {
                for y in chunk {
                    for x in 0..width {
                        let ray = t_cam.read().unwrap().ray_for_pixel(x, y);
                        let color = t_world
                            .read()
                            .unwrap()
                            .color_at(&ray, MAXIMUM_REFLECTION_RECURSION_DEPTH);
                        let _ = t_tx.send((x, y, color));
                    }
                }
            });
        }
        drop(tx);
        for (x, y, color) in rx.iter() {
            let _ = canvas.draw(x, y, color).map_err(|e| println!("{}", e));
        }
        canvas
    }
}
