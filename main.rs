extern crate tracer;
use tracer::*;
use std::f64::INFINITY;

fn main() {
    // hi();
    // return;

    let mut scene = Scene::new(0.001, INFINITY);
    scene.fill_random(22);

    let origin = Point::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(origin, lookat, up)
        .aspect_ratio(3.0 / 2.0)
        .vertical_fov(20.0)
        .image_width(1200)
        .antialiasing(500)
        .focal_length(10.0)
        .aperture(0.1)
        .max_recursion(50);
    cam.render(&scene);
}
