pub mod rtweekend {
    use rand::Rng;
    // use std::f32::consts::PI;

    // Utility Functions
    // pub fn degrees_to_radians(degrees: f32) -> f32 {
    //     return degrees * PI / 180.0;
    // }

    pub fn random_f32() -> f32 {
        rand::thread_rng().gen_range(0.0..1.0)
    }

    pub fn random_f32_from_range(min: f32, max: f32) -> f32 {
        rand::thread_rng().gen_range(min..max)
    }

    pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
        if x < min {
            return min;
        } else if x > max {
            return max;
        } else {
            return x;
        }
    }
}
