use crate::{Ray, Point, Vec3, Color};

pub trait Hittable {
    fn hit(&self, ray: &Ray, scene: &Scene, min_t: f64, max_t: f64, recursions: usize) -> Option<(Color, HitRecord)>;
}

pub struct Scene<'a> {
    min_t: f64,
    max_t: f64,
    objs: Vec<&'a dyn Hittable>,
}

impl<'a> Scene<'a> {
    pub fn new(min_t: f64, max_t: f64) -> Scene<'a> {
        Scene {min_t, max_t, objs: Vec::new()}
    }

    pub fn add(&mut self, obj: &'a dyn Hittable) {
        self.objs.push(obj);
    }

    pub fn hit(&self, ray: &Ray, depth: usize) -> Option<Color> {
        if depth == 0 {
            return Some(Color::new(0.0, 0.0, 0.0))
        }
        self.objs
            .iter()
            .fold((None, self.max_t), |(cur_hit, cur_max_t), o| {
                let hit = o.hit(ray, self, self.min_t, cur_max_t, depth);
                if let Some((color, hr)) = hit {
                    (Some(color), hr.t)
                } else {
                    (cur_hit, cur_max_t)
                }
            })
            .0
    }

    pub fn bg_color(&self, dir: &Vec3) -> Color {
        let y = 0.5 * (dir.unit().get_y() + 1.0);
        (1.0 - y) * Color::new(1.0, 1.0, 1.0) + y * Color::new(0.5, 0.7, 1.0)
    }
}

pub enum ColorBehavior {
    Normal,
    Color(Color),
    Diffuse,
    LambertDiffuse(Color),
    Reflect(Color, f64),
    Dielectric(f64),
}

pub struct Sphere {
    center: Point,
    radius: f64,
    coloring: ColorBehavior,
}

pub struct HitRecord {
    p: Point,
    front_face: bool,
    normal: Vec3,
    t: f64,
    ray_dir: Vec3,
}

impl HitRecord {
    pub fn refract_by(&self, refr_ratio: f64) -> Vec3 {
        let refr_ratio = if self.front_face {
            1.0 / refr_ratio
        } else {
            refr_ratio
        };
        let cos_theta = -Vec3::dot(&self.ray_dir.unit(), &self.normal);
        let r_out_perp = refr_ratio * (self.ray_dir.unit() + cos_theta * self.normal);
        let r_out_par = -((1.0 - r_out_perp.len_sq()).abs()).sqrt() * self.normal;
        r_out_perp + r_out_par
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64, coloring: ColorBehavior) -> Sphere {
        Sphere{center, radius, coloring}
    }

    pub fn hit_at(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.dir, &ray.dir);
        let b = 2.0 * Vec3::dot(&ray.dir, &oc);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discr = b*b - 4.0*a*c;
        if discr >= 0.0 {
            let solution = (-b - discr.sqrt()) / (2.0 * a);
            if solution >= t_min && solution <= t_max {
                return Some(self.hit_record(ray, solution))
            }
            let solution = (-b + discr.sqrt()) / (2.0 * a);
            if solution >= t_min && solution <= t_max {
                return Some(self.hit_record(ray, solution))
            }
        }
        None
    }

    fn hit_record(&self, ray: &Ray, t: f64) -> HitRecord {
        let p = ray.at(t);
        let outward_normal = (p - self.center).unit();
        let (normal, front_face) = if Vec3::dot(&outward_normal, &ray.dir) > 0.0 {
            // ray in the same dir as the outward normal, so it comes from inside
            (-1.0 * outward_normal, false)
        } else {
            // ray in the opposite dir from the outward normal, so it comes from outside
            (outward_normal, true)
        };
        HitRecord {
            p,
            front_face,
            normal,
            t,
            ray_dir: ray.dir,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, scene: &Scene, min_t: f64, max_t: f64, depth: usize) -> Option<(Color, HitRecord)> {
        self.hit_at(ray, min_t, max_t).map(|hr| {
            let color = match self.coloring {
                ColorBehavior::Normal => 0.5 * (hr.normal + 1.0),
                ColorBehavior::Color(color) => color,
                ColorBehavior::Diffuse => {
                    let new_dir = hr.normal + Vec3::random_in_unit();
                    let ray = Ray::new(hr.p, new_dir);
                    let color = scene.hit(&ray, depth-1).unwrap_or_else(|| scene.bg_color(&new_dir));
                    0.5 * color
                },
               ColorBehavior::LambertDiffuse(attenuation) => {
                   let new_dir = hr.normal + Vec3::random_unit();
                   let ray = Ray::new(hr.p, new_dir);
                   let color = scene.hit(&ray, depth-1).unwrap_or_else(|| scene.bg_color(&new_dir));
                   attenuation * color
               },
                ColorBehavior::Reflect(attenuation, fuzz) => {
                    let new_dir_offset = Vec3::dot(&hr.normal, &ray.dir) * hr.normal;
                    let new_dir = ray.dir - 2.0*new_dir_offset + fuzz * Vec3::random_in_unit();
                    let ray = Ray::new(hr.p, new_dir);
                    let color = scene.hit(&ray, depth-1).unwrap_or_else(|| scene.bg_color(&new_dir));
                    attenuation * color
                },
                ColorBehavior::Dielectric(refract_idx) => {
                    let ray = Ray::new(hr.p, hr.refract_by(refract_idx));
                    scene.hit(&ray, depth-1).unwrap_or_else(|| scene.bg_color(&ray.dir))
                },
            };
            (color, hr)
        })
    }
}
