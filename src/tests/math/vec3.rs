use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use crate::linear_algebra::{utils::TINY_FLOAT, vec3::*};

#[wasm_bindgen_test]
pub fn test_dot() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let ab = Vec3 {
        x: 4.0,
        y: 10.0,
        z: 18.0,
    };
    assert_eq!(Vec3::dot(&a, &b), ab);
}

#[wasm_bindgen_test]
pub fn test_cross() {
    let a = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vec3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let axb = Vec3 {
        x: -3.0,
        y: 6.0,
        z: -3.0,
    };
    assert_eq!(Vec3::cross(&a, &b), axb);
}

#[wasm_bindgen_test]
pub fn test_normalize() {
    let mut a = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut b = Vec3 {
        x: 0.1,
        y: 0.2,
        z: 0.3,
    };
    assert!((a.normalize().len() - 1.0).abs() < TINY_FLOAT);
    assert!((b.normalize().len() - 1.0).abs() < TINY_FLOAT);
}

#[wasm_bindgen_test]
pub fn test_len() {
    let a = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let b = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    assert!((a.len() - (3.0f32).sqrt()).abs() < TINY_FLOAT);
    assert!(b.len().abs() < TINY_FLOAT);
}
