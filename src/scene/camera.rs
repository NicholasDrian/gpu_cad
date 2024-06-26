use std::time::Instant;

use crate::linear_algebra::{mat4::Mat4, vec3::*};
use crate::logging::*;

use super::camera_interface::{CameraDescriptor, CameraType};

pub struct Camera {
    position: Vec3,
    focal_point: Vec3,
    up: Vec3,
    /// Vertical field of view
    fovy: f32,
    aspect: f32,
    /// Closest distance that is rendered
    near_dist: f32,
    /// Farthest distance that is rendered
    far_dist: f32,
    /// Set to none when out of date
    view_proj: Mat4,
    camera_type: CameraType,
    last_frame_time: Option<Instant>,
    is_turning_right: bool,
    is_turning_left: bool,
    is_moving_forward: bool,
    is_moving_backward: bool,
    is_looking_up: bool,
    is_looking_down: bool,
    is_moving_left: bool,
    is_moving_right: bool,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(CameraDescriptor::default())
    }
}

//TODO: toggle for auto motion... eventually
impl Camera {
    pub fn new(params: CameraDescriptor) -> Self {
        let mut res = Self {
            position: params.position,
            focal_point: params.focal_point,
            fovy: params.fovy,
            aspect: params.aspect,
            near_dist: params.near_dist,
            far_dist: params.far_dist,
            up: params.up,
            camera_type: params.camera_type,
            view_proj: Mat4::identity(),
            last_frame_time: None,
            is_turning_right: false,
            is_turning_left: false,
            is_moving_forward: false,
            is_moving_backward: false,
            is_looking_up: false,
            is_looking_down: false,
            is_moving_left: false,
            is_moving_right: false,
        };
        res.tick();
        res
    }

    pub fn tick(&mut self) {
        //TODO:: handle motion
        self.update_view_proj();
    }

    pub fn get_view_proj(&self) -> Mat4 {
        self.view_proj
    }

    pub fn set_camera_type(&mut self, camera_type: CameraType) {
        self.camera_type = camera_type;
    }

    pub fn turn_up(&mut self, theta: f32) -> &mut Self {
        log("YAY");
        match (self.camera_type) {
            CameraType::CAD => {}
            CameraType::FPS => {}
        }
        self
    }
    pub fn look_right(&mut self, theta: f32) -> &mut Self {
        match (self.camera_type) {
            CameraType::CAD => {
                todo!();
            }
            CameraType::FPS => {
                todo!();
            }
        }
        self
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        todo!();
        self
    }

    pub fn zoom(&mut self, scale: f32) -> &mut Self {
        match (self.camera_type) {
            CameraType::CAD => {
                // Move camera closer to focal point
                todo!();
            }
            CameraType::FPS => {
                // Move focal point closer to camera
                todo!();
            }
        }
        self
    }

    fn update_view_proj(&mut self) {
        let view = Mat4::look_at(&self.position, &self.focal_point, &self.up);
        let proj = Mat4::perspective(self.fovy, self.aspect, self.near_dist, self.far_dist);
        self.view_proj = Mat4::multiply(&proj, &view);
    }
}
