pub mod camera {
    use crate::ray::ray::Ray;
    use crate::vec3::point::Point3d;

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    pub struct Camera {
        origin: Point3d,
        lower_left_corner: Point3d,
        horizontal: Point3d,
        vertical: Point3d,
    }

    impl Camera {
        pub fn new() -> Camera {
            let origin = Point3d::new(0.0, 0.0, 0.0);
            let horizontal = Point3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
            let vertical = Point3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
            let origin_projection = Point3d::new(0.0, 0.0, FOCAL_LENGTH);
            let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - origin_projection;
            Camera {
                origin: origin,
                vertical: vertical,
                horizontal: horizontal,
                lower_left_corner: lower_left_corner,
            }
        }

        pub fn get_ray(&self, u: f32, v: f32) -> Ray {
            Ray {
                orig: self.origin,
                dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            }
        }
    }
}
