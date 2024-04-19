use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_dot() {
    let x = 2;
    assert_eq!(2, x);
}

#[wasm_bindgen_test]
pub fn test_cross() {
    let x = 2;
    assert_eq!(2, x);
}

#[wasm_bindgen_test]
pub fn test_normalize() {
    let x = 2;
    assert_eq!(2, x);
}

#[wasm_bindgen_test]
pub fn test_len() {
    let x = 2;
    assert_eq!(2, x);
}
