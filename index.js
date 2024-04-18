import { greet, code_that_throws, run } from './pkg';

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
