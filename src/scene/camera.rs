use crate::linear_algebra::{mat4::Mat4, vec3::*};
pub struct Camera {
    position: Vec3,
    focal_point: Vec3,
    up: Vec3,
    fovy: f32,
    aspect: f32,
    near_dist: f32,
    far_dist: f32,
}

pub type Radians = f32;

impl Camera {
    pub fn new(
        position: Vec3,
        focal_point: Vec3,
        fovy: f32,
        aspect: f32,
        near_dist: f32,
        far_dist: f32,
    ) -> Self {
        todo!();
        //        Self { position, focal_point, fovy, aspect, near_dist, far_dist, up, }
    }

    pub fn get_view_proj(self) -> Mat4 {
        todo!();
    }
}
