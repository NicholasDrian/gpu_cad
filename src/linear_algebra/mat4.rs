use crate::linear_algebra::vec4::Vec4;

/// column major 4x4 matrix.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
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

    pub fn multiply(a: &Mat4, b: &Mat4) -> Mat4 {
        let a00 = a.nums[0];
        let a01 = a.nums[1];
        let a02 = a.nums[2];
        let a03 = a.nums[3];
        let a10 = a.nums[4];
        let a11 = a.nums[5];
        let a12 = a.nums[6];
        let a13 = a.nums[7];
        let a20 = a.nums[8];
        let a21 = a.nums[9];
        let a22 = a.nums[10];
        let a23 = a.nums[11];
        let a30 = a.nums[12];
        let a31 = a.nums[13];
        let a32 = a.nums[14];
        let a33 = a.nums[15];

        let b00 = b.nums[0];
        let b01 = b.nums[1];
        let b02 = b.nums[2];
        let b03 = b.nums[3];
        let b10 = b.nums[4];
        let b11 = b.nums[5];
        let b12 = b.nums[6];
        let b13 = b.nums[7];
        let b20 = b.nums[8];
        let b21 = b.nums[9];
        let b22 = b.nums[10];
        let b23 = b.nums[11];
        let b30 = b.nums[12];
        let b31 = b.nums[13];
        let b32 = b.nums[14];
        let b33 = b.nums[15];

        Mat4 {
            nums: [
                a00 * b00 + a10 * b01 + a20 * b02 + a30 * b03,
                a01 * b00 + a11 * b01 + a21 * b02 + a31 * b03,
                a02 * b00 + a12 * b01 + a22 * b02 + a32 * b03,
                a03 * b00 + a13 * b01 + a23 * b02 + a33 * b03,
                a00 * b10 + a10 * b11 + a20 * b12 + a30 * b13,
                a01 * b10 + a11 * b11 + a21 * b12 + a31 * b13,
                a02 * b10 + a12 * b11 + a22 * b12 + a32 * b13,
                a03 * b10 + a13 * b11 + a23 * b12 + a33 * b13,
                a00 * b20 + a10 * b21 + a20 * b22 + a30 * b23,
                a01 * b20 + a11 * b21 + a21 * b22 + a31 * b23,
                a02 * b20 + a12 * b21 + a22 * b22 + a32 * b23,
                a03 * b20 + a13 * b21 + a23 * b22 + a33 * b23,
                a00 * b30 + a10 * b31 + a20 * b32 + a30 * b33,
                a01 * b30 + a11 * b31 + a21 * b32 + a31 * b33,
                a02 * b30 + a12 * b31 + a22 * b32 + a32 * b33,
                a03 * b30 + a13 * b31 + a23 * b32 + a33 * b33,
            ],
        }
    }

    pub fn transform(self, v: &Vec4) -> Vec4 {
        let v00 = self.nums[0];
        let v01 = self.nums[4];
        let v02 = self.nums[8];
        let v03 = self.nums[12];
        let v10 = self.nums[1];
        let v11 = self.nums[5];
        let v12 = self.nums[9];
        let v13 = self.nums[13];
        let v20 = self.nums[2];
        let v21 = self.nums[6];
        let v22 = self.nums[10];
        let v23 = self.nums[14];
        let v30 = self.nums[3];
        let v31 = self.nums[7];
        let v32 = self.nums[11];
        let v33 = self.nums[15];

        Vec4 {
            x: v.x * v00 + v.y * v01 + v.z * v02 + v.w * v03,
            y: v.x * v10 + v.y * v11 + v.z * v12 + v.w * v13,
            z: v.x * v20 + v.y * v21 + v.z * v22 + v.w * v23,
            w: v.x * v30 + v.y * v31 + v.z * v32 + v.w * v33,
        }
    }
}
