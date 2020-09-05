extern crate tracer;
use tracer::*;

fn main() {
    let mut scene = Scene::new(0.0, 5.0);
    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    scene.add(&sphere);
    let sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);
    scene.add(&sphere);

    render(&scene);
}
