use crate::SCENES;
use wasm_bindgen::prelude::*;

use super::{camera_interface::CameraDescriptor, handles::Handle, scene::*};

/// TODO: add error handling
#[wasm_bindgen]
pub fn set_camera(scene: Handle, descriptor: CameraDescriptor) -> Result<(), JsValue> {
    let mut scenes = SCENES.lock().unwrap();
    (*scenes).get_mut(&scene).unwrap().set_camera(descriptor);
    Ok(())
}

#[wasm_bindgen]
pub fn turn_camera_up(scene: Handle, theta: f32) -> Result<(), JsValue> {
    let mut scenes = SCENES.lock().unwrap();
    (*scenes)
        .get_mut(&scene)
        .unwrap()
        .get_camera_mut()
        .turn_up(theta);
    Ok(())
}
