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
export function readMemSlice(wasm, prt, View) {
    ptr = ptr >>> 0;
    const [sPtr, len] = readSliceHeader(wasm, ptr);
    const bytes = new View(getMem(wasm), sPtr, len).slice();
    wasm.exports.free(ptr);
    return bytes;
}
