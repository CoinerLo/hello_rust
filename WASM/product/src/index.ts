import { Order } from "../pkg/product.js";

import { ProductsView, ProductsViewRaw } from "./view.ts";

const order = new Order(42n, "Bob");

for (let i = 1; i <= 500000; i++) {
    const sku = `PROD-${String(i).padStart(3, "0")}`;
    const price = (9.99 + i * 2.3).toFixed(2);
    const quantity = Math.max(0, 50 - (i % 23));
    const in_stock = quantity > 0 && i % 6 !== 0;

    order.addProduct(sku, Number(price), quantity, in_stock);
}

console.time("getItemsJS");
console.log(order.getItemsJS()[100].sku);
console.timeEnd("getItemsJS");

console.time("getItemsJSON");
console.log(order.getItemsJSON()[100].sku);
console.timeEnd("getItemsJSON");

console.time("getItemsBinary");
console.log(new ProductsView(order.getItemsBinary()).get(100).sku);
console.timeEnd("getItemsBinary");

console.time("getItemsBinaryRaw");
console.log(new ProductsViewRaw(order.getItemsBinaryRaw()).get(100).sku);
console.timeEnd("getItemsBinaryRaw");
