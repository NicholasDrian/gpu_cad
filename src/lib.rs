pub mod geometry;
pub mod linear_algebra;
pub mod logging;
pub mod render;
pub mod scene;
pub mod widgets;

#[cfg(test)]
pub mod tests;

// for global static mut
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

use render::renderer::Renderer;
use scene::handles::{new_handle, Handle};
use scene::scene::Scene;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use winit::dpi::PhysicalSize;
use winit::platform::web::{WindowBuilderExtWebSys, WindowExtWebSys};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

lazy_static! {
    static ref SCENES: Mutex<HashMap<Handle, Scene>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn new_scene() -> Handle {
    let handle = new_handle();
    let scene = Scene::default();
    let mut scenes_changer = SCENES.lock().unwrap();
    (*scenes_changer).insert(handle, scene);
    handle
}

// This is run once on module load
#[wasm_bindgen(start)]
pub fn init() {}

#[wasm_bindgen]
pub async fn run(canvas: HtmlCanvasElement, scene_handle: Handle) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

    window.set_inner_size(PhysicalSize::new(1200, 850));

    let mut renderer = Renderer::new(window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.window().id() => {
                if !renderer.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            renderer.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == renderer.window().id() => {
                let mut scenes_changer = SCENES.lock().unwrap();
                let scene: &mut Scene = (*scenes_changer).get_mut(&scene_handle).unwrap();
                scene.tick();
                match renderer.render(scene) {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        //renderer.resize(renderer.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                renderer.window().request_redraw();
            }
            _ => {}
        }
    });
}
