import { add, fib, Product } from '../pkg';


const resultOne = add(1, 2);

const resultTwo = fib(4);

const product = new Product();

const label = document.getElementById('result');

label.innerHTML = `${resultOne}`;

console.log('Result from wasm.fib: ', resultTwo);

console.log('Result from wasm.Product', product);

