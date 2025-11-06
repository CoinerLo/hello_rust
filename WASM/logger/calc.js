import { getSum } from "./getSum.wasm";

function sum(a, b) {
    getSum(a, b);
}

sum(1, 3);
