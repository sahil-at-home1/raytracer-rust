use crate::hittable::hittable::{HitRecord, Hittable};
use crate::ray::ray::Ray;
use crate::vec3::point::Point3d;

pub struct Sphere {
    center: Point3d,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Point3d = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        // don't deal with imaginary numbers
        if discriminant < 0.0 {
            return false;
        }
        // find nearest root that lies in range
        let sqrtd: f32 = discriminant.sqrt();
        let mut root: f32 = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        // update hit record
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }
}

impl Sphere {
    pub fn new(center: Point3d, radius: f32) -> Self {
        Sphere { center, radius }
    }
}
