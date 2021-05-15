# Basic Example

This is a basic example but a lot is going on here,

Inside of our Rust lib we create two functions, Add, Fib, and then we also create a Struct with some methods.

We then use wasm-pack to build our Rust source code into a WASM module.

We import the module to our JS_Side of things by directly importing the folder that wasm-pack generated as this is a module.

Note: Look into the webpack configuration, we are required to use an experimental feature if we wish to import it Async like how I have inside of main.js

Also note how we can use the Struct we created in Rust as a contructor for a Class in JS, and it will produce a valid object with the methods we had given it in our impl


_______
Example... This Struct

```
#[wasm_bindgen]
pub struct Product {
    id: i32,
}

#[wasm_bindgen]
impl Product {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Product {
        Product { id: val }
    }

    pub fn get(&self) -> i32 {
        self.id
    }

    pub fn set(&mut self, val: i32) {
        self.id = val;
    }
}
```

Becomes a constructor for a class inside of JS,
Remeber I was talking about the functions generated in the background, well here is what the actual export

After compolation, inside /pkg/basic_example_bg.js

```
export class Product {

    static __wrap(ptr) {
        const obj = Object.create(Product.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_product_free(ptr);
    }
    /**
    * @param {number} val
    */
    constructor(val) {
        var ret = wasm.product_new(val);
        return Product.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    get() {
        var ret = wasm.product_get(this.ptr);
        return ret;
    }
    /**
    * @param {number} val
    */
    set(val) {
        wasm.product_set(this.ptr, val);
    }
}
```
_____

To acheive functionality like..

```
  import { add, fib, Product } from '../pkg';


  const product = new Product(2);
  product.set(45);

  console.log(product.get())

  // => 45



  const resultAdd = add(2, 3);

  console.log(resultAdd);

  // => 5



  const resultFib = fib(4);

  console.log(resultFib);

  // => 4

```