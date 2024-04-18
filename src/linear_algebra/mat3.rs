use crate::linear_algebra::vec3::Vec3;

/// column major 3x3 matrix.
#[derive(Debug)]
pub struct Mat3 {
    pub nums: [f32; 9],
}

impl Mat3 {
    pub fn identity() -> Mat3 {
        Mat3 {
            nums: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn mul(a: Mat3, b: Mat3) -> Mat3 {
        todo!();
    }

    pub fn transform(self, v: Vec3) -> Vec3 {
        todo!();
    }
}
