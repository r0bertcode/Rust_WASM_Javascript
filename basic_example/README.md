# Basic Example

This is a basic example but a lot is going on here,

Inside of our Rust lib we create two functions, Add, Fib, and then we also create a Struct with some methods.

We then use wasm-pack to build our Rust source code into a WASM module.

We import the module to our JS_Side of things by directly importing the folder that wasm-pack generated as this is a module.

Note: Look into the webpack configuration, we are required to use an experimental feature if we wish to import it Async like how I have inside of main.js

Also note how we can use the Struct we created in Rust as a contructor for an Object in JS, and it will produce a valid object with the methods we had given it
in our impl
