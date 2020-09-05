extern crate tracer;
use tracer::*;
use tracer::ColorBehavior;
use std::f64::INFINITY;

fn main() {
    let mut scene = Scene::new(0.001, INFINITY);
    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, ColorBehavior::DIFFUSE);
    scene.add(&sphere);
    let sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ColorBehavior::DIFFUSE);
    scene.add(&sphere);

    render(&scene);
}
