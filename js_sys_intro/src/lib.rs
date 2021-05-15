use wasm_bindgen::prelude::*;

/*

    Notice: we are using the js_sys crate this allows
    us defined JS types unlike using JsType or &JsType

    js_sys::Array is exactly what you would expect a js arr to be like,
    they even include a lot of the native methods JS arrays have

    Here we work with slice, and length and below
    on the last function we use the new word to
    generate a new empty array

*/

#[wasm_bindgen]
pub fn first(array: js_sys::Array, n: u32) -> js_sys::Array {
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