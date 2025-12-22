/**
 * @typedef {Uint32Array} USIZE_VIEW
 */

/**
 * Размер usize (32, так как wasm32-unknown-unknown)
 * @type {Uint32ArrayConstructor}
 */
const USIZE = Uint32Array;

/**
 * Разыменовывает линейную память заданного Wasm экземпляра
 *
 * @param {WebAssembly.Instance} wasm
 * @returns {ArrayBuffer}
 */
export function getMem(wasm) {
    return wasm.exports.memory.buffer;
}

/**
 * Считывает значение заголовка среза из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {Uint32Array}
 */
export function readSliceHeader(wasm, ptr) {
    ptr = ptr >>> 0;
    return new Uint32Array(getMem(wasm), ptr, 2);
}

/**
 * @typedef {
 *   Uint8ArrayConstructor|
 *   Int8ArrayConstructor|
 *   Uint16ArrayConstructor|
 *   Int16ArrayConstructor|
 *   Uint32ArrayConstructor|
 *   Int32ArrayConstructor|
 *   BigUint64ArrayConstructor|
 *   BigInt64ArrayConstructor|
 *   Float32ArrayConstructor|
 *   Float64ArrayConstructor
 * } ViewConstructor
 */

/**
 * @typedef {ArrayBufferView & ArrayLike} View
 */

/**
 * Считывает значение среза из линейной памяти заданного Wasm экземпляра по указателю с использованием указанной проекции
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @param {ViewConstructor} View
 * @returns {ArrayBufferView|*}
 */
export function readMemSlice(wasm, ptr, View) {
    ptr = ptr >>> 0;
    const [sPtr, len] = readSliceHeader(wasm, ptr);
    const bytes = new View(getMem(wasm), sPtr, len).slice();
    wasm.exports.free(ptr);
    return bytes;
}

/**
 * Записывает данные в линейную память заданного Wasm экземпляра в виде среза
 *
 * @param {WebAssembly.Instance} wasm
 * @param {View} data любой типизированный массив
 * @returns {[number, number]} кортеж, где первый элемент указатель, а второй - длина (согласно типу)
 */
export function packSlice(wasm, data) {
    const ptr = wasm.exports.malloc(data.byteLength);
    const mem = getMem(wasm);
    new Uint8Array(mem, ptr, data.byteLength).set(new Uint8Array(data.buffer));
    return [ptr, data.length];
}

/**
 * Записывает заголовочные данные среза в линейную память заданного Wasm экземпляра
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @param {number} len
 * @returns {number} указатель на заголовок в линейной памяти
 */
export function packHeader(wasm, ptr, len) {
    const hPtr = wasm.exports.malloc(2 * USIZE.BYTES_PER_ELEMENT);
    const mem = getMem(wasm);
    new USIZE(mem, hPtr, 2).set(new USIZE([ptr, len]));
    return hPtr;
}

/**
 * Записывает строку в линейную память заданного Wasm экземпляра в виде среза
 *
 * @param {WebAssembly.Instance} wasm
 * @param {string} str
 * @returns {[number,number]} кортеж, где первый элемент указатель, а второй - длина (согласно типу)
 */
export function packStr(wasm, str) {
    return packSlice(wasm, new TextDecoder().encode(str));
}

/**
 * Записывает строку в линейную память заданного Wasm экземпляра в виде среза
 *
 * @param {WebAssembly.Instance} wasm
 * @param {string} str
 * @returns {number} указатель на заголовок среза
 */
export function packStrWithHeader(wasm, str) {
    return packHeader(wasm, ...packStr(wasm, str));
}

/**
 * Считывает строку из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {string}
 */
export function readStr(wasm, ptr) {
    return new TextDecoder().decode(readMemSlice(wasm, ptr, Uint8Array));
}

/**
 * Считывает массив строк из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {string[]}
 */
export function readStrs(wasm, ptr) {
    const strs = [];
    readSliceUSize(wasm, ptr).forEach((ptr) => {
        strs.push(readStr(wasm, ptr));
    })
    return strs;
}

/**
 * Считывает usize срез из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {USIZE_VIEW}
 */
export function readSliceUSize(wasm, ptr) {
    return readMemSlice(wasm, ptr, USIZE);
}

/**
 * Считывает i32 срез из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {Int32Array}
 */
export function readSliceI32(wasm, ptr) {
    return readMemSlice(wasm, ptr, Int32Array);
}

/**
 * Считывает i64 срез из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {BigInt64Array}
 */
export function readSliceI64(wasm, ptr) {
    return readMemSlice(wasm, ptr, BigInt64Array);
}

/**
 * Считывает f32 срез из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {Float32Array}
 */
export function readSliceF32(wasm, ptr) {
    return readMemSlice(wasm, ptr, Float32Array);
}

/**
 * Считывает f64 срез из линейной памяти заданного Wasm экземпляра по указателю
 *
 * @param {WebAssembly.Instance} wasm
 * @param {number} ptr
 * @returns {Float64Array}
 */
export function readSliceF64(wasm, ptr) {
    return readMemSlice(wasm, ptr, Float64Array);
}
