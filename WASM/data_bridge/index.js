import * as fs from "node:fs";
import { LocalStorage } from "node-localstorage";
import * as lib from "./lib.js";

const localStorage = new LocalStorage("./scratch");

const { instance: wasm } = await WebAssembly.instantiate(fs.readFileSync("./target/wasm32-unknown-unknown/release/data_bridge.wasm"), {
    env: {
        lsGetItem(key) {
            const data = localStorage.getItem(lib.readStr(wasm, key) ?? "");
            return lib.packStrWithHeader(wasm, data);
        },
        lsSetItem(key, value) {
            localStorage.setItem(lib.readStr(wasm, key), lib.readStr(wasm, value));
        },
        lsRemoveItem(key) {
            localStorage.removeItem(lib.readStr(wasm, key));
        }
    }
});

const {
    get_str,
    mod_str,
    get_strs,
    get_slice_f32,
    mod_vec_f32,
    get_slice_i64,
    write_something_to_ls,
    read_something_from_ls,
} = wasm.exports;

console.log(lib.readStr(wasm, get_str()));
console.log(lib.readStrs(wasm, get_strs()));

console.log(lib.readSliceF32(wasm, get_slice_f32()));
console.log(lib.readSliceI64(wasm, get_slice_i64()));

console.log(lib.readSliceF32(wasm, mod_vec_f32(...lib.packSlice(wasm, new Float32Array([42.53, 11.3])))));
console.log(lib.readStr(wasm, mod_str(...lib.packStr(wasm, "Привет из JS"))));

write_something_to_ls();
console.log(lib.readStr(wasm, read_something_from_ls()));
