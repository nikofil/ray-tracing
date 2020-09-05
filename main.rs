use std::fs::File;
use std::io::prelude::*;

extern crate tracer;
use tracer::*;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;

fn write_to_file(colors: &[[Color; HEIGHT]; WIDTH]) -> std::io::Result<()> {
    let mut f = File::create("out.ppm")?;
    let head: String = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    f.write_all(head.as_bytes())?;

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            f.write_all((colors[i][j].to_s() + "\n").as_bytes())?;
        }
    }

    Ok(())
}

fn main() {
    let mut colors = [[Color::new(0.0, 0.0, 0.0); HEIGHT]; WIDTH];

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let viewport_vec = Vec3::new(0.0, 0.0, -FOCAL_LENGTH);
    let viewport_upper_left = origin - horizontal/2.0 + vertical/2.0 + viewport_vec;

    let mut scene = Scene::new();
    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    scene.add(&sphere);

    for j in 0..HEIGHT {
        eprint!("\rLine {} of {}", j+1, HEIGHT);
        let y = (j as f64) / (HEIGHT as f64 - 1.0);
        for i in 0..WIDTH {
            let x = (i as f64) / (WIDTH as f64 - 1.0);
            let ray = Ray::new(origin, viewport_upper_left + x * horizontal - y * vertical - origin);
            colors[i][j] = ray.ray_color(&scene);
        }
    }
    eprint!("\n");

    write_to_file(&colors).unwrap();
    let mut x = Point::new(1.0,2.0,3.0);
    x += Point::new(1.1, 2.2, 3.3);
}
