# Rust_WASM_Javascript
Repo I'm using while I explore wasm-bindgen and JS as a way to get familiar with Rust and WASM


# How dose it work?

We write source code in Rust as a library, using the wasm-bindgen trait on anything we create in Rust ( Functions, Impl's, Struct's... )

Cargo ( Rust's package manager and more ) has a package called ' wasm-pack ' that will actually build your rust lib into a WASM node package and then we can use that as a module inside Javascript and during exectuion or when we create this package we get a lot of helper funtions generated for us in the background as applying the wasm-bindgen trait added on a lot of great implementations to handle the messy stuff,

This mainly includes things like creating heaps or stacks in the background to manage memory and managing the JsTypes and data, functions on both sides to help with conversions between langage or between WASM and Rust or calling them, and defering types, as WASM can't store anything but number types and on the Rust side, we don't have access to things like JsObjects

So we are given these wonderful functions and types inside rust itself when we add the wasm-bindgen trait, here: https://rustwasm.github.io/wasm-bindgen/introduction.html is wonderful documentation going much deeper into this and everything you need to know to get started.

And after all that complicated messy stuff is safely handled in the background when we are executing these functions and converting the types or giving us pointers so we can access the memeory, we are able to export these functions through WASM to the javascript enviorment and use them as a module.

Example:

Rust Side

```
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  pub fn multiply(num1: i32, num2: i32) -> i32 {
      num1 * num2
  }  
```

JS Side
```
  // Pkg is the folder generated from wasm-pack
  import * as wasm from '../pkg';

  const result = wasm.multiplty(2, 8);

  const btn = document.getElementById('btn');
  
  btn.addEventListener('click', () => {
     console.log('This result is from a wasm module', result);
  });
  
  // => This result is from a wasm module 16
```

Or look through some of the repos here to learn more.
