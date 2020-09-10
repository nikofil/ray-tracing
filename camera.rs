extern crate rand;

use crate::{Color, Scene, Ray, Vec3, Point, new_executor_and_spawner};
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
    aperture: f64,
    image_width: usize,
    max_recursion: u32,
}

impl Camera {
    pub fn new(pos: Point, lookat: Point, up: Vec3) -> Camera {
        Camera {
            pos,
            lookat,
            up: up.unit(),
            aspect_ratio: 16.0 / 9.0,
            vertical_fov: PI / 2.0,
            antialiasing: 10,
            focal_len: 1.0,
            aperture: 0.0,
            image_width: 400,
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

    pub fn aperture(mut self, aperture: f64) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn image_width(mut self, width: usize) -> Self {
        self.image_width = width;
        self
    }

    pub fn max_recursion(mut self, max_rec: u32) -> Self {
        self.max_recursion = max_rec;
        self
    }

    fn write_out(&self, colors: &Vec<Vec<Color>>, height: usize) {
        print!("P3\n{} {}\n255\n", self.image_width, height);

        for j in 0..height {
            for i in 0..self.image_width {
                print!("{}\n", colors[i][j].to_s());
            }
        }
    }

    fn aperture_offset(&self, right: Vec3, up: Vec3) -> Vec3 {
        let offset_weight = (self.aperture / 2.0) * Vec3::random_in_unit();
        offset_weight.get_x() * right + offset_weight.get_y() * up
    }

    pub fn render(&self, scene: &Scene) {
        let image_width = self.image_width;
        let image_height = (image_width as f64 / self.aspect_ratio) as usize;
        let mut colors = vec![vec![Color::new(0.0, 0.0, 0.0); image_height]; image_width];

        let dir = (self.lookat - self.pos).unit();
        let viewport_height = (self.vertical_fov / 2.0).tan() * 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;

        let right_vec = Vec3::cross(&dir, &self.up).unit();
        let up_vec = Vec3::cross(&right_vec, &dir).unit();

        let horizontal = self.focal_len * viewport_width * right_vec;
        let vertical = self.focal_len * viewport_height * up_vec;

        let viewport_vec = self.focal_len * dir;
        let viewport_upper_left = self.pos - horizontal / 2.0 + vertical / 2.0 + viewport_vec;

        let pos = self.pos;
        let antialiasing = self.antialiasing;
        let max_recursion = self.max_recursion;
        let (executor, spawner) = new_executor_and_spawner();

        for j in 0..image_height {
            eprint!("\rLine {} of {}", j + 1, image_height);
            let y = (j as f64) / (image_height as f64 - 1.0);
            for i in 0..image_width {
                spawner.spawn(async {
                    let mut rng = rand::thread_rng();

                    let x = (i as f64) / (image_width as f64 - 1.0);
                    let start_pos = pos;//  + self.aperture_offset(right_vec, up_vec);
                    let dir = viewport_upper_left + x * horizontal - y * vertical - pos;
                    let ray = Ray::new(start_pos, dir);
                    // colors[i][j] = ray.ray_color(&scene, max_recursion);
                    for _ in 0..antialiasing {
                        let dx = rng.gen_range(0.0, 1.0) / (image_width as f64 - 1.0);
                        let dy = rng.gen_range(0.0, 1.0) / (image_height as f64 - 1.0);
                        let start_pos = pos;// + aperture_offset(right_vec, up_vec);
                        let dir = viewport_upper_left + (x + dx) * horizontal - (y + dy) * vertical - start_pos;
                        let ray = Ray::new(start_pos, dir);
                        // colors[i][j] += ray.ray_color(&scene, max_recursion);
                    }
                    // colors[i][j] /= antialiasing as f64 + 1.0;
                    colors[i][j] = colors[i][j].sqrt()
                });
            }
        }
        eprint!("\n");

        executor.run();

        self.write_out(&colors, image_height);
    }
}
