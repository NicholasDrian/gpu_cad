use crate::linear_algebra::vec4::Vec4;

use super::vec3::Vec3;

/// column major 4x4 matrix.
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, bytemuck::Zeroable, bytemuck::Pod)]
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

    pub fn look_at(position: &Vec3, focal_point: &Vec3, up: &Vec3) -> Mat4 {
        // Z Axis is forward, not up
        let z_axis = *Vec3::subtract(position, focal_point).normalize();
        let x_axis = *Vec3::cross(up, &z_axis).normalize();
        let y_axis = Vec3::cross(&z_axis, &x_axis);
        Mat4 {
            nums: [
                x_axis.x,
                y_axis.x,
                z_axis.x,
                0.0,
                x_axis.y,
                y_axis.y,
                z_axis.y,
                0.0,
                x_axis.z,
                y_axis.z,
                z_axis.z,
                0.0,
                -Vec3::dot(&x_axis, position),
                -Vec3::dot(&y_axis, position),
                -Vec3::dot(&z_axis, position),
                1.0,
            ],
        }
    }

    /// z_far can be infinity
    pub fn perspective(fovy: f32, aspect: f32, near_dist: f32, far_dist: f32) -> Mat4 {
        let temp = f32::tan((std::f32::consts::PI - fovy) / 2.0);
        Mat4 {
            nums: [
                temp / aspect,
                0.0,
                0.0,
                0.0,
                0.0,
                temp,
                0.0,
                0.0,
                0.0,
                0.0,
                if far_dist == f32::INFINITY {
                    -1.0
                } else {
                    far_dist / (near_dist - far_dist)
                },
                -1.0,
                0.0,
                0.0,
                if far_dist == f32::INFINITY {
                    -near_dist
                } else {
                    far_dist * near_dist / (near_dist - far_dist)
                },
                0.0,
            ],
        }
    }

    pub fn translation(t: &Vec3) -> Mat4 {
        Mat4 {
            nums: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, t.x, t.y, t.z, 1.0,
            ],
        }
    }

    pub fn rotation(axis: &Vec3, theta: f32) -> Mat4 {
        todo!();
        /*
            let x = axis.x;
          let y = axis.y;
          let z = axis.z;
          const n = Math.sqrt(x * x + y * y + z * z);
          x /= n;
          y /= n;
          z /= n;
          const xx = x * x;
          const yy = y * y;
          const zz = z * z;
          const c = Math.cos(angleInRadians);
          const s = Math.sin(angleInRadians);
          const oneMinusCosine = 1 - c;

          dst[ 0] = xx + (1 - xx) * c;
          dst[ 1] = x * y * oneMinusCosine + z * s;
          dst[ 2] = x * z * oneMinusCosine - y * s;
          dst[ 3] = 0;
          dst[ 4] = x * y * oneMinusCosine - z * s;
          dst[ 5] = yy + (1 - yy) * c;
          dst[ 6] = y * z * oneMinusCosine + x * s;
          dst[ 7] = 0;
          dst[ 8] = x * z * oneMinusCosine + y * s;
          dst[ 9] = y * z * oneMinusCosine - x * s;
          dst[10] = zz + (1 - zz) * c;
          dst[11] = 0;
          dst[12] = 0;
          dst[13] = 0;
          dst[14] = 0;
          dst[15] = 1;
        */
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

    pub fn transform_point(&self, p: &Vec3) -> Vec3 {
        let temp = self.transform(&Vec4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        });
        Vec3 {
            x: temp.x / temp.w,
            y: temp.y / temp.w,
            z: temp.z / temp.w,
        }
    }

    pub fn transform_vector(&self, v: &Vec3) -> Vec3 {
        let temp = self.transform(&Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        });
        Vec3 {
            x: temp.x / temp.w,
            y: temp.y / temp.w,
            z: temp.z / temp.w,
        }
    }
}
