use wasm_bindgen::prelude::*;

use crate::linear_algebra::vec3::Vec3;

#[wasm_bindgen]
pub struct CameraDescriptor {
    pub position: Vec3,
    pub focal_point: Vec3,
    pub up: Vec3, //TODO: remove this param
    pub fovy: f32,
    pub aspect: f32,
    pub near_dist: f32,
    pub far_dist: f32,
    pub camera_type: CameraType,
}

#[wasm_bindgen]
impl CameraDescriptor {
    #[wasm_bindgen(constructor)]
    pub fn new(
        position: Vec3,
        focal_point: Vec3,
        up: Vec3, //TODO: remove this param
        fovy: f32,
        aspect: f32,
        near_dist: f32,
        far_dist: f32,
        camera_type: CameraType,
    ) -> Self {
        Self {
            position,
            focal_point,
            up, //TODO: remove this param
            fovy,
            aspect,
            near_dist,
            far_dist,
            camera_type,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum CameraType {
    /// This is a first person shooter style camera.
    /// Rotation is around the cameras position
    FPS,
    /// This is a CAD style camera.
    /// Rotation is around the focal point
    CAD,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            position: Vec3 {
                x: 0.0,
                y: 1.0,
                z: -10.0,
            },
            focal_point: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            fovy: 1.0,
            aspect: 1.0,
            near_dist: 0.01,
            far_dist: 10000.0,
            camera_type: CameraType::CAD,
        }
    }
}
