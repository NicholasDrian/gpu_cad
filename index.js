import { Vec3, CameraDescriptor, set_camera, new_scene, greet, code_that_throws, run, CameraType } from './pkg';

let scene_handle = new_scene();
console.log(scene_handle);

let camera_descriptor = new CameraDescriptor(
  new Vec3(0.0, 1.0, -10),
  new Vec3(0.0, 1.0, -10),
  new Vec3(0.0, 1.0, -10),
  1.0,
  1.0,
  1.0,
  1.0,
  CameraType.CAD
)
let set_camera_result = set_camera(scene_handle, camera_descriptor);
console.log(camera_descriptor, set_camera_result);


let res = greet('World');
console.log(res);

res = code_that_throws();
console.log(res);

var canvas = document.createElement('canvas');
document.body.appendChild(canvas);

var div = document.createElement('div');
document.body.appendChild(div);
div.id = "wasm-example";

run(); 
