use crate::linear_algebra::vec4::Vec4;

/// column major 3x3 matrix.
#[derive(Debug)]
pub struct Mat4 {
    pub nums: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        Mat4 {
            nums: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn mul(a: Mat4, b: Mat4) -> Mat4 {
        todo!();
    }

    pub fn transform(self, v: Vec4) -> Vec4 {
        todo!();
    }
}
