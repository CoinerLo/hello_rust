use product::*;

fn main() {
    let mut order = Order::new(42, "Bob");

    order.add_product("PROD-100", 239.99, 42, true);
    order.add_product("PROD-101", 242.29, 41, true);

    assert_eq!(482.28, order.total_value());
    assert_eq!(2, order.total_items_count());
}
