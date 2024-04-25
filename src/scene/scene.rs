use std::collections::HashMap;

use crate::geometry::{curve::Curve, surface::Surface};

use super::camera::Camera;

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

    pub fn tick(&mut self) {
        self.camera.tick();
    }
}
