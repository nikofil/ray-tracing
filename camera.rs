extern crate rand;

use crate::{Color, Scene, Ray, Vec3, Point};
use std::fs::File;
use std::io::prelude::*;
use self::rand::Rng;

// Image
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 600;
pub const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

// Camera
pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
pub const FOCAL_LENGTH: f64 = 1.0;
pub const ANTIALISING: usize = 15;
pub const MAX_DEPTH: usize = 25;

fn write_to_file(colors: &Vec<Vec<Color>>) -> std::io::Result<()> {
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

pub fn render(origin: Point, scene: &Scene) {
    let mut colors = vec![vec![Color::new(0.0, 0.0, 0.0); HEIGHT]; WIDTH];
    let mut rng = rand::thread_rng();

    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let viewport_vec = Vec3::new(0.0, 0.0, -FOCAL_LENGTH);
    let viewport_upper_left = origin - horizontal/2.0 + vertical/2.0 + viewport_vec;

    for j in 0..HEIGHT {
        eprint!("\rLine {} of {}", j + 1, HEIGHT);
        let y = (j as f64) / (HEIGHT as f64 - 1.0);
        for i in 0..WIDTH {
            let x = (i as f64) / (WIDTH as f64 - 1.0);
            let ray = Ray::new(origin, viewport_upper_left + x * horizontal - y * vertical - origin);
            colors[i][j] = ray.ray_color(&scene);
            for _ in 0..ANTIALISING {
                let dx = rng.gen_range(0.0, 1.0) / (WIDTH as f64 - 1.0);
                let dy = rng.gen_range(0.0, 1.0) / (HEIGHT as f64 - 1.0);
                let ray = Ray::new(origin, viewport_upper_left + (x+dx) * horizontal - (y+dy) * vertical - origin);
                colors[i][j] += ray.ray_color(&scene);
            }
            colors[i][j] /= ANTIALISING as f64 + 1.0;
            colors[i][j] = colors[i][j].sqrt()
        }
    }
    eprint!("\n");

    write_to_file(&colors).unwrap();
}
