#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn dot(a: &Vec4, b: &Vec4) -> Vec4 {
        Vec4 {
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z,
            w: a.w * b.w,
        }
    }
}
