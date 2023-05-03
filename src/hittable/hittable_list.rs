use crate::hittable::hittable::{HitRecord, Hittable};
use crate::ray::ray::Ray;
// use crate::vec3::point::Point3d;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        let v: Vec<Box<dyn Hittable>> = Vec::new();
        HittableList { objects: v }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }
        // move the closest hit record out
        if hit_anything {
            *rec = temp_rec;
        }

        return hit_anything;
    }
}
