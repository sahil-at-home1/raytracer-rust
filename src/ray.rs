pub mod ray {
    use crate::vec3::point::Point3d;

    pub struct Ray {
        pub orig: Point3d,
        pub dir: Point3d,
    }

    impl Ray {
        pub fn new(orig: Point3d, dir: Point3d) -> Ray {
            Ray { orig, dir }
        }

        // P(t) = A + tb
        pub fn at(&self, t: f32) -> Point3d {
            self.orig + self.dir * t
        }
    }
}
