use crate::ray::ray::Ray;
use crate::vec3::point::Point3d;

pub struct HitRecord {
    pub p: Point3d,
    pub normal: Point3d,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3d::new(0.0, 0.0, 0.0),
            normal: Point3d::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Point3d) {
        self.front_face = r.dir.dot(*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = *outward_normal * -1.0
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
