extern crate rand;

use crate::{Color, Scene, Ray, Vec3, Point};
use self::rand::Rng;
use std::f64::consts::PI;

pub struct Camera {
    pos: Point,
    lookat: Vec3,
    up: Vec3,
    aspect_ratio: f64,
    vertical_fov: f64,
    antialiasing: u32,
    focal_len: f64,
    image_width: usize,
    image_height: usize,
    max_recursion: u32,
}

impl Camera {
    pub fn new(pos: Point, lookat: Point, up: Vec3) -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        Camera {
            pos,
            lookat,
            up: up.unit(),
            aspect_ratio,
            vertical_fov: PI / 2.0,
            antialiasing: 10,
            focal_len: 1.0,
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as usize,
            max_recursion: 10,
        }
    }

    pub fn aspect_ratio(mut self, ar: f64) -> Self {
        self.aspect_ratio = ar;
        self
    }

    pub fn vertical_fov(mut self, fov: f64) -> Self {
        self.vertical_fov = fov.to_radians();
        self
    }

    pub fn antialiasing(mut self, aa: u32) -> Self {
        self.antialiasing = aa;
        self
    }

    pub fn focal_length(mut self, fl: f64) -> Self {
        self.focal_len = fl;
        self
    }

    pub fn image_width(mut self, width: usize) -> Self {
        self.image_width = width;
        self.image_height = (width as f64 / self.aspect_ratio) as usize;
        self
    }

    pub fn max_recursion(mut self, max_rec: u32) -> Self {
        self.max_recursion = max_rec;
        self
    }

    fn write_out(&self, colors: &Vec<Vec<Color>>) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                print!("{}\n", colors[i][j].to_s());
            }
        }
    }

    pub fn render(&self, scene: &Scene) {
        let mut colors = vec![vec![Color::new(0.0, 0.0, 0.0); self.image_height]; self.image_width];
        let mut rng = rand::thread_rng();

        let viewport_height = (self.vertical_fov / 2.0).tan() * 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;
        let dir = (self.lookat - self.pos).unit();

        let horizontal = viewport_width * Vec3::cross(&dir, &self.up).unit();
        let vertical = viewport_height * Vec3::cross(&horizontal, &dir).unit();
        let viewport_vec = self.focal_len * dir;
        let viewport_upper_left = self.pos - horizontal / 2.0 + vertical / 2.0 + viewport_vec;

        for j in 0..self.image_height {
            eprint!("\rLine {} of {}", j + 1, self.image_height);
            let y = (j as f64) / (self.image_height as f64 - 1.0);
            for i in 0..self.image_width {
                let x = (i as f64) / (self.image_width as f64 - 1.0);
                let ray = Ray::new(self.pos, viewport_upper_left + x * horizontal - y * vertical - self.pos);
                colors[i][j] = ray.ray_color(&scene, self.max_recursion);
                for _ in 0..self.antialiasing {
                    let dx = rng.gen_range(0.0, 1.0) / (self.image_width as f64 - 1.0);
                    let dy = rng.gen_range(0.0, 1.0) / (self.image_height as f64 - 1.0);
                    let ray = Ray::new(self.pos, viewport_upper_left + (x + dx) * horizontal - (y + dy) * vertical - self.pos);
                    colors[i][j] += ray.ray_color(&scene, self.max_recursion);
                }
                colors[i][j] /= self.antialiasing as f64 + 1.0;
                colors[i][j] = colors[i][j].sqrt()
            }
        }
        eprint!("\n");

        self.write_out(&colors);
    }
}
