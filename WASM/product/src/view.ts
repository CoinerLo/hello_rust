import * as wasm from "../pkg/product_bg.wasm";

const memory: WebAssembly.Memory = wasm.memory;

export type Area = (byteOffset?: number) => [number, number];

export class ProductView {
    buffer: ArrayBuffer | SharedArrayBuffer;
    byteOffset: number;

    static get sku(): (byteOffset?: number) => [number, number, number] {
        return (byteOffset = 0) => [byteOffset, byteOffset + 8, byteOffset + 4];
    }

    static get price(): Area {
        return (byteOffset = 0) => {
            const [_, from] = this.sku(byteOffset);
            return [from, from + 8];
        }
    }

    static get quantity(): Area {
        return (byteOffset = 0) => {
            const [_, from] = this.price(byteOffset);
            return [from, from + 4];
        }
    }

    static get inStock(): Area {
        return (byteOffset = 0) => {
            const [_, from] = this.quantity(byteOffset);
            return [from, from + 1];
        }
    }

    static get size() {
        return this.inStock()[1] - this.sku()[0];
    }

    get helpers() {
        return this.constructor as typeof ProductView;
    }

    get sku() {
        const [ptrFrom, _, lenFrom] = this.helpers.sku(this.byteOffset);

        const ref = new Uint32Array(this.buffer.slice(ptrFrom, ptrFrom + 4))[0];

        if (ref == null) {
            throw new Error("Null pointer exception");
        }

        const len = new Uint32Array(this.buffer.slice(lenFrom, lenFrom + 4))[0] ?? 0;

        return new TextDecoder().decode(memory.buffer.slice(ref, ref + len));
    }

    get price() {
        return new Float64Array(this.buffer.slice(...this.helpers.price(this.byteOffset)))[0] ?? 0;
    }

    get quantity() {
        return new Uint32Array(this.buffer.slice(...this.helpers.quantity(this.byteOffset)))[0] ?? 0;
    }

    get inStock() {
        return !!new Uint8Array(this.buffer.slice(...this.helpers.inStock(this.byteOffset)))[0];
    }

    constructor(bytes: ArrayBufferLike | ArrayBufferView, byteOffset: number) {
        this.buffer = ArrayBuffer.isView(bytes) ? bytes.buffer : bytes;
        this.byteOffset = byteOffset;
    }
}

export class ProductsView {
    static BYTES_PER_ELEMENT = ProductView.size;

    buffer: ArrayBuffer | SharedArrayBuffer;
    byteOffset = 0;

    constructor(bytes: ArrayBufferLike | ArrayBufferView) {
        if (ArrayBuffer.isView(bytes)) {
            this.buffer = bytes.buffer;
            this.byteOffset = bytes.byteOffset;

        } else {
            this.buffer = bytes;
        }
    }

    get(index: number) {
        return new ProductView(this.buffer, this.byteOffset + index * ProductsView.BYTES_PER_ELEMENT);
    }
}

export class ProductViewRaw extends ProductView {
    static override get sku(): typeof ProductView.sku {
        return (byteOffset = 0) => [byteOffset + 4, byteOffset + 16, byteOffset + 8];
    }
}

export class ProductsViewRaw {
    static BYTES_PER_ELEMENT = ProductViewRaw.size + 7;

    buffer: ArrayBuffer | SharedArrayBuffer;
    byteOffset: number;

    constructor(header: Uint32Array) {
        const bytes = new Uint8Array(memory.buffer, ...header);
        this.buffer = bytes.buffer;
        this.byteOffset = bytes.byteOffset;
    }

    get(index: number) {
        return new ProductViewRaw(this.buffer, this.byteOffset + index * ProductsViewRaw.BYTES_PER_ELEMENT);
    }
}