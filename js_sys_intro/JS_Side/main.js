import { first, last } from '../pkg';

const array = [1, 3, 3, 7, 1, 3, 3, 7];

console.log('This is a leet WASM first call: ', first(array, 4));

console.log('This is a leet WASM last call: ', last(array, 4));

// Console Logs:
// => This is a leet WASM first call:  (4) [1, 3, 3, 7]
// => This is a leet WASM last call:  (4) [1, 3, 3, 7]
