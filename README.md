# Rust_WASM_Javascript
Repo I'm using while I explore wasm-bindgen and JS as a way to get familiar with Rust and WASM

Almost all of the content here can be found on the offical docs, this is more meant to document my progression,
and to make source code avaliable for others trying to learn how to implement WASM with Rust to JS envoirments.

Each folder will be it's own ' mini - project ' and will include a README going over the core concepts of that project

# How dose it work?

We write source code in Rust as a library, using the wasm-bindgen trait on anything we create in Rust ( Functions, Impl's, Struct's... )

Implementning this trait will give us additional helper functions as methods or in the background, mainly inside the JS enviorment, this icludes things like: Stacks/Heaps and helper functions to manage the pointers and memory or type conversions since WASM cannot accept types like String, so it must be converted, and thankfully all of this is done for us in the backgrond for anything valid that we added the wasm-bindgen trait upon.
( Much better resource to understand all of what goes on behind the curtains: https://rustwasm.github.io/wasm-bindgen/ )

We bundle this source code into a WASM node module, wasm-pack will do just that for you, generating a node package containing all of the relevant WASM files.

We then import this as a module into a JS enviorment and use it like we would any other.
____
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

  console.log('This result is from a wasm module', result);

  // => This result is from a wasm module 16
```

____

