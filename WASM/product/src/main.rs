pub struct Product {
    sku: String,          // Артикул, например "TSH-BLU-M"
    price: f64,           // Цена за единицу
    quantity: u32,        // Количество
    in_stock: bool,       // Есть ли на складе
}

struct Order {
    order_id: u64,
    customer_name: String,
    created_at: String,   // ISO 8601, например "2025-01-14T13:45:00Z"
    items: Vec<Product>,
}

impl Order {
    /// Конструктор — создаёт новый заказ
    pub fn new(order_id: u64, customer_name: String) -> Order;

    /// Добавляет товар в заказ
    pub fn add_product(&mut self, sku: String, price: f64, quantity: u32, in_stock: bool);

    /// Количество позиций в заказе
    pub fn total_items_count(&self) -> usize;

    /// Общая стоимость (∑ price × quantity)
    pub fn total_value(&self) -> f64;

    /// Вариант А — возвращает массив объектов JS
    pub fn get_items_js(&self) -> JsValue;

    /// Вариант Б — возвращает бинарные данные
    pub fn get_items_binary(&self) -> Vec<u8>;
}

fn main() {
    println!("Hello, world!");
}
