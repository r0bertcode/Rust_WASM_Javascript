use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn first(array: js_sys::Array, n: u32) -> js_sys::Array {
    if n == 0 {
        return js_sys::Array::new();
    }

    array.slice(0, n)
}

#[wasm_bindgen]
pub fn last(array: js_sys::Array, n: u32) -> js_sys::Array {
    if n > array.length() {
        return array;
    }
    if n == 0 {
        return js_sys::Array::new();
    }

    array.slice(n, array.length())
}