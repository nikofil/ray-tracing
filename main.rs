extern crate tracer;
use tracer::*;
use tracer::ColorBehavior;
use std::f64::INFINITY;

fn main() {
    let mut scene = Scene::new(0.001, INFINITY);
    let spheres = [
        Sphere::new( // small center
            Point::new(0.0, -0.3, -0.5),
            0.1,
            ColorBehavior::Normal),
        Sphere::new( // up
            Point::new(0.0, 1.0, -0.8),
            0.5,
            ColorBehavior::Reflect(Color::new(0.8, 0.8, 0.9), 0.8)),
        Sphere::new( // right
            Point::new(0.5, 0.0, -1.0),
            0.5,
            ColorBehavior::Reflect(Color::new(0.8, 0.8, 0.9), 0.1)),
        Sphere::new( // left
            Point::new(-0.5, 0.0, -1.0),
            0.5,
            ColorBehavior::LambertDiffuse(Color::new(0.8, -0.1, 0.2))),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ColorBehavior::Diffuse),
    ];
    spheres.iter().for_each(|s| scene.add(s));

    let origin = Point::new(0.0, 0.0, 0.0);
    render(origin, &scene);
}
