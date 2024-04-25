use std::collections::HashMap;

use crate::geometry::{curve::Curve, surface::Surface};

use super::{camera::Camera, camera_interface::CameraDescriptor};

pub type UUID = u32;

pub struct Scene {
    camera: Camera,
    curves: HashMap<UUID, Curve>,
    surfaces: HashMap<UUID, Surface>,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            camera: Camera::default(),
            curves: HashMap::new(),
            surfaces: HashMap::new(),
        }
    }
}

impl Scene {
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn set_camera(&mut self, descriptor: CameraDescriptor) -> &mut Self {
        self.camera = Camera::new(descriptor);
        self
    }

    pub fn tick(&mut self) {
        self.camera.tick();
    }
}
