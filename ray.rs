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

    pub fn ray_color(&self, scene: &Scene) -> Color {
        if scene.hit(self) {
            Color::new(1.0, 0.0, 0.0)
        } else {
            let y = 0.5 * (self.dir.unit().get_y() + 1.0);
            (1.0 - y) * Color::new(1.0, 1.0, 1.0) + y * Color::new(0.5, 0.7, 1.0)
        }
    }
}
