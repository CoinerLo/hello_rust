import { getFactorial } from "./getFactorial.wasm";

console.log(getFactorial(5));  // 120
console.log(getFactorial(0));  // 1 (поскольку цикл не запускается)
console.log(getFactorial(1));  // 1
