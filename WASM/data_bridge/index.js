import * as fs from "node:fs";
import * as lib from "./lib";

const { instance: wasm } = await WebAssembly.instantiate(fs.readFileSync("./target/wasm32-unknown-unknown/release/demo.wasm"), {});

const {
    get_str,
    mod_str,
    get_strs,
    get_slice_f32,
    mod_vec_f32,
    get_slice_i64,
} = wasm.exports;

console.log(lib.readStr(wasm, get_str()));
console.log(lib.readStrs(wasm, get_strs()));

console.log(lib.readSliceF32(wasm, get_slice_f32()));
console.log(lib.readSliceI64(wasm, get_slice_i64()));

console.log(lib.readSliceF32(wasm, mod_vec_f32(...lib.packSlice(wasm, new Float32Array([42.53, 11.3])))));
console.log(lib.readStr(wasm, mod_str(...lib.packStr(wasm, "Привет из JS"))));
