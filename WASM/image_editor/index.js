import initWasm, { wasm_resize, wasm_auto_resize } from "./dist/image_editor.js";
import { resize } from "./resize.js";

await initWasm();

const canvas = document.getElementById("editor");
const ctx = canvas.getContext("2d");
let uploadedIamge;

document.getElementById("image").addEventListener("input", async (e) => {
    const file = e.target.files[0];
    const img = new Image();
    img.src = URL.createObjectURL(file);
    if (!img.complete) {
        await new Promise((r) => img.onload = r);
    }

    const {elements: { srcw, srch }} = document.getElementById("image-params");
    canvas.width = img.width;
    canvas.height = img.height;

    ctx.drawImage(img, 0, 0);
    srcw.value = img.width;
    srch.value = img.height;

    uploadedIamge = ctx.getImageData(0, 0, canvas.width, canvas.height);
    URL.revokeObjectURL(file);
})

document.getElementById("reset").addEventListener("click", (e) => {
    e.preventDefault();
    if (uploadedIamge) {
        canvas.width = uploadedIamge.width;
        canvas.height = uploadedIamge.height;
        ctx.putImageData(uploadedIamge, 0, 0);
    }
})

document.getElementById("image-params").addEventListener("submit", (e) => {
    console.time("resize");
    e.preventDefault();
    const { type, width, height, sharpness } = e.target.elements;

    const newWidth = parseInt(width.value);
    const newHeight = parseInt(height.value);

    const image = ctx.getImageData(0, 0, canvas.width, canvas.height);

    let imageData;

    if (type.value === "wasm") {
        console.log("Масштабирование wasm модулем");
        const bytes = wasm_resize(
            image.data,
            image.width,
            image.height,
            newWidth,
            newHeight,
            parseInt(sharpness.value),
        );

        imageData = new ImageData(
            new Uint8ClampedArray(bytes.buffer),
            newWidth,
            newHeight,
        );
    } else {
        console.log("Масштабирование js модулем");
        imageData = resize(
            ctx.getImageData(0, 0, canvas.width, canvas.height),
            newWidth,
            newHeight,
            parseInt(sharpness.value),
        );
    }

    canvas.width = newWidth;
    canvas.height = newHeight;
    ctx.putImageData(imageData, 0, 0);
    console.log("Масштабирование завершено");
    console.timeEnd("resize");
})

document.getElementById("wasm-resize").addEventListener("click", (e) => {
    e.preventDefault();
    wasm_auto_resize();
})
