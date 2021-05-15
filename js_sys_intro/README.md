# JS_SYS

A way to interact with JS objects, methods, properties through bindings
______________________________

This Rust library with two functions, First and Last

As seen here in src/lib.rs

```
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

```

Notice how we are using a method called slice, on an array, sounds very similar to JS dosen't it? JS_sys has bindings to JS standard, built in objects, including their methods and properties. ( Not including Node.js, Web, or JS envoirment API's )

This demeonstrates how we can program a function in rust that takaes in a specific JS type, and we can even use the methods we would normally be able to use inside JS as well, or we can do things like converting them into Vectors and then returning a js_sys::Array,

________
Here is how things will look on the JS_Side

```
import { first, last } from '../pkg';

const array = [1, 3, 3, 7, 1, 3, 3, 7];

console.log('This is a leet WASM first call: ', first(array, 4));

console.log('This is a leet WASM last call: ', last(array, 4));

// => This is a leet WASM first call:  (4) [1, 3, 3, 7]
// => This is a leet WASM last call:  (4) [1, 3, 3, 7]

```

This gives us the ability to create implementations that uses both Js Types and Rust types, and allows us easier access between the two languages.