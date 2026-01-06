/**
 * Масштабирует заданное изображение методом Ланцоша и возвращает новое
 *
 * @param {ImageData} srcImageData - исходные данные изображения (из ctx.getImageData)
 * @param {number} dstWidth - новая ширина
 * @param {number} dstHeight - новая высота
 * @param {number} r - радиус Ланцоша (влияет на качество)
 * @returns {ImageData}
 */
export function resize(srcImageData, dstWidth, dstHeight, r = 3) {
    if (dstWidth <= 0 || dstHeight <= 0) throw new Error("Destination dimensions must be positive");
    if (srcImageData.width <= 0 || srcImageData.height <= 0) throw new Error("Source image must have positive dimensions");

    const srcWidth = srcImageData.width;
    const srcHeight = srcImageData.height;
    const srcData = srcImageData.data;

    const support = r - 1;
    const kernelSize = 2 * r - 1;

    // Создаём симметричное ядро Lanczos
    const kernel = new Float32Array(kernelSize);

    for (let i = 0; i < kernelSize; i++) {
        const x = i - (r - 1);
        kernel[i] = lanczosKernel(x, r);
    }

    // Промежуточный буфер: ресайз по ширине
    const intermediate = new ImageData(dstWidth, srcHeight);
    resizeHorizontal(srcData, intermediate.data, srcWidth, srcHeight, dstWidth, kernel, support);

    // Финальный результат: ресайз по высоте
    const dst = new ImageData(dstWidth, dstHeight);
    resizeVertical(intermediate.data, dst.data, dstWidth, srcHeight, dstHeight, kernel, support);

    return dst;
}

/**
 * Горизонтальный проход
 */
function resizeHorizontal(srcData, dstData, srcWidth, srcHeight, dstWidth, kernel, support) {
    const scaleX = srcWidth / dstWidth;

    for (let y = 0; y < srcHeight; y++) {
        const srcBase = y * srcWidth * 4;
        const dstBase = y * dstWidth * 4;

        for (let dstX = 0; dstX < dstWidth; dstX++) {
            const srcXCenter = (dstX + 0.5) * scaleX;

            const start = Math.floor(srcXCenter - support);
            const end = Math.floor(srcXCenter + support);

            let sumR = 0, sumG = 0, sumB = 0, sumA = 0, sumW = 0;

            for (let srcXI = start; srcXI <= end; srcXI++) {
                let srcX = Math.max(0, Math.min(srcWidth - 1, srcXI));
                let relI = srcXI - start;
                if (relI >= kernel.length) continue;

                const weight = kernel[relI];
                if (Math.abs(weight) < 1e-5) continue;

                const idx = srcBase + srcX * 4;

                sumR += srcData[idx] * weight;
                sumG += srcData[idx + 1] * weight;
                sumB += srcData[idx + 2] * weight;
                sumA += srcData[idx + 3] * weight;
                sumW += weight;
            }

            const outIdx = dstBase + dstX * 4;

            if (sumW > 1e-5) {
                const invW = 1 / sumW;
                dstData[outIdx]     = sumR * invW + 0.5;
                dstData[outIdx + 1] = sumG * invW + 0.5;
                dstData[outIdx + 2] = sumB * invW + 0.5;
                dstData[outIdx + 3] = sumA * invW + 0.5;

            } else {
                let srcX = Math.round(srcXCenter);
                srcX = Math.max(0, Math.min(srcWidth - 1, srcX));

                const idx = srcBase + srcX * 4;
                dstData[outIdx]     = srcData[idx];
                dstData[outIdx + 1] = srcData[idx + 1];
                dstData[outIdx + 2] = srcData[idx + 2];
                dstData[outIdx + 3] = srcData[idx + 3];
            }
        }
    }
}

/**
 * Вертикальный проход
 */
function resizeVertical(srcData, dstData, width, srcHeight, dstHeight, kernel, support) {
    const scaleY = srcHeight / dstHeight;

    for (let x = 0; x < width; x++) {
        const colOffset = x * 4;

        for (let dstY = 0; dstY < dstHeight; dstY++) {
            const srcYCenter = (dstY + 0.5) * scaleY;

            const start = Math.floor(srcYCenter - support);
            const end = Math.floor(srcYCenter + support);

            let sumR = 0, sumG = 0, sumB = 0, sumA = 0, sumW = 0;

            for (let srcYI = start; srcYI <= end; srcYI++) {
                let srcY = Math.max(0, Math.min(srcHeight - 1, srcYI));
                let relI = srcYI - start;
                if (relI >= kernel.length) continue;

                const weight = kernel[relI];
                if (Math.abs(weight) < 1e-5) continue;

                const idx = srcY * width * 4 + colOffset;

                sumR += srcData[idx] * weight;
                sumG += srcData[idx + 1] * weight;
                sumB += srcData[idx + 2] * weight;
                sumA += srcData[idx + 3] * weight;
                sumW += weight;
            }

            const outIdx = dstY * width * 4 + colOffset;

            if (sumW > 1e-5) {
                const invW = 1 / sumW;
                dstData[outIdx]     = sumR * invW + 0.5;
                dstData[outIdx + 1] = sumG * invW + 0.5;
                dstData[outIdx + 2] = sumB * invW + 0.5;
                dstData[outIdx + 3] = sumA * invW + 0.5;

            } else {
                let srcY = Math.round(srcYCenter);
                srcY = Math.max(0, Math.min(srcHeight - 1, srcY));

                const idx = srcY * width * 4 + colOffset;
                dstData[outIdx]     = srcData[idx];
                dstData[outIdx + 1] = srcData[idx + 1];
                dstData[outIdx + 2] = srcData[idx + 2];
                dstData[outIdx + 3] = srcData[idx + 3];
            }
        }
    }
}

/**
 * Рассчет ядра Ланцоша
 */
function lanczosKernel(x, radius) {
    if (x === 0) return 1.0;

    const absX = Math.abs(x);
    if (absX >= radius) return 0.0;

    const piX = Math.PI * absX;
    const r = radius;
    return r * Math.sin(piX) * Math.sin(piX / r) / (piX * piX);
}