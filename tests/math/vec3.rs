use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn correct_function() {
    let x = 2;
    assert_eq!(2, x);
}

#[wasm_bindgen_test]
pub fn correct_function1() {
    let x = 2;
    assert_eq!(2, x);
}

#[wasm_bindgen_test]
pub fn correct_function2() {
    let x = 2;
    assert_eq!(2, x);
}
