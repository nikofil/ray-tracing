use crate::{Ray, Point, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> bool;
}

pub struct Scene<'a> {
    objs: Vec<&'a dyn Hittable>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {objs: Vec::new()}
    }

    pub fn add(&mut self, obj: &'a dyn Hittable) {
        self.objs.push(obj);
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        self.objs.iter().find(|o| o.hit(ray)).is_some()
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.dir, &ray.dir);
        let b = 2.0 * Vec3::dot(&ray.dir, &oc);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discr = b*b - 4.0*a*c;
        return discr >= 0.0
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere{center, radius}
    }
}
