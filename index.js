import { Vec3, CameraDescriptor, set_camera, new_scene, greet, code_that_throws, run, CameraType, turn_camera_up } from './pkg';

let scene_handle = new_scene();
console.log(scene_handle);

let camera_descriptor = new CameraDescriptor(
  new Vec3(0.0, 1.0, -10),
  new Vec3(0.0, 1.0, 0.0),
  new Vec3(0.0, 1.0, 0.0),
  1.0,
  1280.0 / 800,
  0.01,
  10000.0,
  CameraType.CAD
)
set_camera(scene_handle, camera_descriptor);
turn_camera_up(scene_handle, 0.1)


let res = greet('World');
console.log(res);

res = code_that_throws();
console.log(res);

var canvas = document.createElement('canvas');
document.body.appendChild(canvas);
console.log("canvas", canvas);

var div = document.createElement('div');
document.body.appendChild(div);
div.id = "wasm-example";

run(canvas, scene_handle); 
