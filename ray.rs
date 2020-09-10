use crate::vec::{Point, Color, Vec3};
use crate::scene::Scene;

pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.dir
    }

    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray {origin, dir}
    }

    pub fn ray_color(&self, scene: &Scene, max_depth: u32) -> Color {
        if let Some(color) = scene.hit(self, max_depth) {
            color
        } else {
            scene.bg_color(&self.dir)
        }
    }
}
