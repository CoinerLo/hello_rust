import { Order } from "./pkg/product.js";

const order = new Order(42n, "Bob");

order.addProduct("PROD-001", 42.42, 42, true);
order.addProduct("PROD-002", 134.24, 1, false);

export function getProductSKU(index) {
    return order.getItemsJS()[index].sku;
}

export function getProductSKU2(index) {
    return order.getItemsJS()[index].sku;
}
